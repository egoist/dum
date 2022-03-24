use ansi_term::Color::{Green, Red};
use serde::Deserialize;
#[derive(Deserialize)]
struct Answer {
    tag_name: String,
}

pub fn update_check() -> Result<(), ureq::Error> {
    let answer: Answer = ureq::get("https://api.github.com/repos/egoist/dum/releases/latest")
        .call()?
        .into_json()?;
    let version = answer.tag_name.replace("v", "");
    if env!("CARGO_PKG_VERSION") != version {
        println!(
            "{} {}",
            Red.normal().paint("There is a new version of dum:"),
            Green.normal().paint(version)
        );
    }
    Ok(())
}
