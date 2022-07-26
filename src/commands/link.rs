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

#[derive(PartialEq, Eq, Debug)]
pub enum IssueLinkType {
    Issue(u64),
    PullRequest(u64),
}

fn match_link_issue_or_pr(text: &str) -> Option<IssueLinkType> {
    lazy_static! {
        // From https://github.com/laundmo/gh-linker-bot/blob/main/gh_linker/cogs/code_snippets.py
        // TODO: Find out what multiline compile means
        static ref GH_ISSUE_RE: Regex = Regex::new(r#"((?P<Username>\w+)*/)?(?P<IsUser>@)?(?P<Repo>[^\s]+)#(?P<Id>\d*)"#).unwrap();
    }

    let mut a = GH_ISSUE_RE.captures_iter(text);
    
    let k = a.by_ref().count();
    info!("Captured Matches: {:?}", k);

    for b in a {
        for c in b.iter(){
            info!("{:?}", c);
        }
        
        /*
        info!("Username {:?}", &b["Username"]);
        info!("IsUser {:?}", &b["IsUser"]);
        info!("Repo {:?}", &b["Repo"]);
        info!("Id {:?}", &b["Id"]);
        */
    }

    if GH_ISSUE_RE.is_match(text)
    {
        Some(IssueLinkType::Issue(12))
    } else {
        None
    }
}


#[test]
fn feature() {
    let a = match_link_issue_or_pr("bevy#123");
    assert_eq!(a.expect("Not Found"), IssueLinkType::Issue(12));
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


    let res1 = match_link_issue_or_pr(&msg.content);
    match res1 {
        Some(IssueLinkType::Issue(x)) => info!("Found an issue {}", x),
        Some(IssueLinkType::PullRequest(x)) => info!("Found a pull-request {}", x),
        None => ()
    }

    if res.is_some() {
        msg.channel_id
            .say(&ctx.http, format!("Bonjour {}", lcs))
            .await?;
    }

    Ok(())
}
