//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::Args;
use serenity::framework::standard::Delimiter;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::MessageType;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use tracing::{error, info};

use crate::commands::hello::*;
use crate::commands::link::*;
use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::owner::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        if !msg.author.bot && msg.kind.eq(&MessageType::Regular) {
            //info!("Someone messaged");
            //info!("{:?}", msg);
            let _ = link(&context, &msg, Args::new("", &[Delimiter::Single(';')])).await;
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping, hello, multiply, quit)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    let env_file = dotenv::dotenv();
    match env_file {
        Ok(_) => println!(".env found, setting environment variables"),
        Err(x) => println!("No .env file found: {:?}", x),
    }

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let dc_token = env::var("DISCORD_TOKEN").expect("Expected a discord token in the environment");
    let _db_endpoint =
        env::var("DATABASE_ENDPOINT").expect("Expected a database endpoint in the environment");
    let _gh_token =
        env::var("DATABASE_ENDPOINT").expect("Expected a github token in the environment");

    let http = Http::new(&dc_token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&dc_token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
