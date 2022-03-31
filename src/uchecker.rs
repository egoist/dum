use ansi_term::Color::{Green, Red};
use serde::Deserialize;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Deserialize)]
struct Answer {
    tag_name: String,
}

pub async fn update_check() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let answer = client
        .get("https://api.github.com/repos/egoist/dum/releases/latest")
        .send()
        .await?
        .json::<Answer>()
        .await?;
    let version = answer.tag_name.replace('v', "");
    if env!("CARGO_PKG_VERSION") != version {
        println!(
            "{} {}",
            Red.normal().paint("There is a new version of dum:"),
            Green.normal().paint(version)
        );
    }
    Ok(())
}
