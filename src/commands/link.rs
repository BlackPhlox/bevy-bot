use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[derive(PartialEq, Eq, Debug)]
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

#[test]
fn parse_gh_link() {
    assert_eq!(match_link_code_storage("https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md"), Some(CodeLinkType::GitHub));
}

#[test]
fn parse_gh_gist_link() {
    assert_eq!(match_link_code_storage("https://gist.github.com/BlackPhlox/ed97c5a6b3793ec3f43d59e65acd54b8"), Some(CodeLinkType::GitHubGist));
}

#[test]
fn parse_gitlab_link() {
    assert_eq!(match_link_code_storage("https://gitlab.com/vismut-org/vismut"), Some(CodeLinkType::GitLab));
}

#[test]
fn parse_bitbucket_link() {
    assert_eq!(match_link_code_storage("https://bitbucket.org/atlassian/pipelines-examples-rust"), Some(CodeLinkType::BitBucket));
}

#[derive(PartialEq, Eq, Debug)]
pub struct Issue {
    id: u64,
    repo: Repo,
    author: String,
    issue_type: IssueType,
}

#[derive(PartialEq, Eq, Debug)]
pub enum IssueType {
    Issue,
    PR,
    Discussion,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Repo {
    Bevy,
    BevyBot,
    BevyWeb,
    Other(String), //Username/Repo
}

#[allow(clippy::manual_map)]
fn parse_issue_link(text: &str) -> Option<Issue> {
    lazy_static! {
        // See definition in https://github.com/BlackPhlox/bevy-bot/issues/9
        static ref GH_ISSUE_RE: Regex = Regex::new(r#"((?P<Username>\w+)*/)?(?P<IsUser>@)?(?P<Repo>[^\s]+)#(?P<Id>\d*)"#).unwrap();
    }

    let captures = GH_ISSUE_RE.captures(text);

    if let Some(x) = captures {
        let is_user = &x.name("IsUser");
        if is_user.is_some() {
            return None;
        }

        let id = &x.name("Id");
        let repo = &x.name("Repo");
        let username = &x.name("Username");

        if let (Some(_id), Some(_repo), user) = (id, repo, username) {
            let r: Option<Repo> = match _repo.as_str() {
                "bevy" | "b" => Some(Repo::Bevy),
                "bevy-website" | "website" | "web" => Some(Repo::BevyWeb),
                "bevy-bot" | "bot" => Some(Repo::BevyBot),
                _ => {
                    if let Some(_user) = user {
                        //Github lookup first, if exist user and repo exist return some else none
                        Some(Repo::Other(_repo.as_str().to_string()))
                    } else {
                        None
                    }
                }
            };

            if let Some(r) = r {
                //Call github repo api
                let i_type = match r {
                    Repo::Bevy => IssueType::Issue,
                    Repo::BevyBot => IssueType::PR,
                    Repo::BevyWeb => IssueType::Discussion,
                    Repo::Other(_) => IssueType::Issue,
                };
                println!("Type {:?}", i_type);
                return Some(Issue {
                    id: _id.as_str().to_string().parse::<u64>().expect("Invalid id"),
                    repo: r,
                    //From github
                    author: "cart".to_string(),
                    issue_type: IssueType::Issue,
                });
            }
        }
    }
    None
}

#[test]
fn parse_bevy_issue() {
    assert_eq!(
        parse_issue_link("bevy#123"),
        Some(Issue {
            id: 123,
            repo: Repo::Bevy,
            author: "cart".to_string(),
            issue_type: IssueType::Issue
        })
    );
}

#[test]
fn parse_false_positive_fails() {
    assert_eq!(parse_issue_link("@bevy#1234"), None);
}

#[test]
fn parse_issue_fallback() {
    assert_eq!(
        parse_issue_link("BlackPhlox/bevy_config_cam#1"),
        Some(Issue {
            id: 1,
            repo: Repo::Other("bevy_config_cam".to_string()),
            author: "BlackPhlox".to_string(),
            issue_type: IssueType::Issue
        })
    );
}

#[test]
fn parse_bevy_web_issue() {
    assert_eq!(
        parse_issue_link("web#123"),
        Some(Issue {
            id: 123,
            repo: Repo::BevyWeb,
            author: "cart".to_string(),
            issue_type: IssueType::Issue
        })
    );
}

#[test]
fn parse_bevy_bot_issue() {
    assert_eq!(
        parse_issue_link("bot#9"),
        Some(Issue {
            id: 9,
            repo: Repo::BevyBot,
            author: "BlackPhlox".to_string(),
            issue_type: IssueType::Issue
        })
    );
}

#[test]
fn parse_bevy_pr() {
    assert_eq!(
        parse_issue_link("bevy#123"),
        Some(Issue {
            id: 123,
            repo: Repo::Bevy,
            author: "cart".to_string(),
            issue_type: IssueType::PR
        })
    );
}

#[test]
fn parse_bevy_discussion() {
    assert_eq!(
        parse_issue_link("bot#123"),
        Some(Issue {
            id: 123,
            repo: Repo::BevyBot,
            author: "cart".to_string(),
            issue_type: IssueType::Discussion
        })
    );
}

#[command]
pub async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Checking for link regex!");

    let res = match_link_code_storage(&msg.content);

    if let Some(r) = res {
        let lcs = match r {
            CodeLinkType::GitHub => "GH",
            CodeLinkType::GitHubGist => "GHG",
            CodeLinkType::GitLab => "GL",
            CodeLinkType::BitBucket => "BB",
        };

        msg.channel_id
            .say(&ctx.http, format!("Bonjour {}", lcs))
            .await?;

        return Ok(());
    }

    if let Some(r) = parse_issue_link(&msg.content) {
        match r {
            Issue {
                id,
                repo,
                author,
                issue_type: IssueType::Issue,
            } => info!("Found an issue {} in {:#?} by {}", id, repo, author),
            Issue {
                id,
                repo,
                author,
                issue_type: IssueType::PR,
            } => info!("Found a pull-request {} in {:#?} by {}", id, repo, author),
            Issue {
                id,
                repo,
                author,
                issue_type: IssueType::Discussion,
            } => info!("Found a Discussion {} in {:#?} by {}", id, repo, author),
        }

        return Ok(());
    }

    Ok(())
}
