use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
pub async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push("Hi ")
        .push_bold_safe(&msg.author.name)
        .push_line("! Nice to meet you!")
        .push_line("We're")
        .push_line("**E**velyn\n**C**arter\n**S**am")
        .push_line("We can do a bunch of stuff such as:")
        .push_italic("~ping\n~multiply x y")
        .push_line("")
        .push("Found out more here: https://github.com/BlackPhlox/bevy-bot")
        //.mention(&channel)
        //.push(" channel")
        .build();

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
