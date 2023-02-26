use anyhow::Context;
use serde::Deserialize;
use std::env;
use std::fs;
use urlencoding;
use xshell::{cmd, Shell};

#[derive(Deserialize)]
struct Link {
    url: String,
    title: Option<String>,
    summary: Option<String>,
    starred: bool,
}

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    let pinboard_api_token =
        env::var("PINBOARD_API_TOKEN").with_context(|| "Failed to find $PINBOARD_API_TOKEN")?;

    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])?;

    let links: Vec<Link> = serde_json::from_str(contents.as_str())?;

    for link in links {
        if !link.starred {
            continue;
        }

        let url = urlencoding::encode(link.url.as_str()).to_string();
        let title =
            urlencoding::encode(link.title.unwrap_or("FIXME".to_string()).as_str()).to_string();
        let description =
            urlencoding::encode(link.summary.unwrap_or("".to_string()).as_str()).to_string();

        cmd!(
            sh,
            "https https://api.pinboard.in/v1/posts/add?url={url}&description={title}&extened={description}&auth_token={pinboard_api_token}"
        ).run()?;
    }

    return Ok(());
}
