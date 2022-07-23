use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

pub enum CodeLinkType {
    GitHub,
    GitHubGist,
    GitLab,
    BitBucket,
}

fn match_link_code_storage(text: &str) -> Option<CodeLinkType> {
    lazy_static! {
        // From https://github.com/laundmo/gh-linker-bot/blob/main/gh_linker/cogs/code_snippets.py
        // TODO: Find out what multiline compile means
        static ref GITHUB_RE: Regex = Regex::new(r#"https://github\.com/(?P<repo>[a-zA-Z0-9-]+/[\w.-]+)/blob/"#).unwrap();
        static ref GITHUB_GIST_RE: Regex = Regex::new(r#"https://gist\.github\.com/([a-zA-Z0-9-]+)/(?P<gist_id>[a-zA-Z0-9]+)/*"#).unwrap();
        static ref GITLAB_RE: Regex = Regex::new(r#"https://gitlab\.com/(?P<repo>[\w.-]+/[\w.-]+)/\-/blob/(?P<path>[^#>]+)"#).unwrap();
        static ref BITBUCKET_RE: Regex = Regex::new(r#"https://bitbucket\.org/(?P<repo>[a-zA-Z0-9-]+/[\w.-]+)/src/(?P<ref>[0-9a-zA-Z]+)"#).unwrap();
    }

    // TODO: Convert to a match stmt
    if GITHUB_RE.is_match(text) {
        Some(CodeLinkType::GitHub)
    } else if GITHUB_GIST_RE.is_match(text) {
        Some(CodeLinkType::GitHubGist)
    } else if GITLAB_RE.is_match(text) {
        Some(CodeLinkType::GitLab)
    } else if BITBUCKET_RE.is_match(text) {
        Some(CodeLinkType::BitBucket)
    } else {
        None
    }
}

#[command]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Checking for link regex!");

    let res = match_link_code_storage(&msg.content);
    let lcs = match res {
        Some(ref x) => match x {
            CodeLinkType::GitHub => "GH",
            CodeLinkType::GitHubGist => "GHG",
            CodeLinkType::GitLab => "GL",
            CodeLinkType::BitBucket => "BB",
        },
        None => "",
    };

    if res.is_some() {
        msg.channel_id
            .say(&ctx.http, format!("Bonjour {}", lcs))
            .await?;
    }

    Ok(())
}
