use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;

// A function to guess package manager by looking for lock file in current directory only
// If yarn.lock is found, it's likely to be a yarn project
// If package-lock.json is found, it's likely to be a npm project
// If pnpm-lock.yaml is found, it's likely to be a pnpm project
// If none of the above is found, return None
pub fn guess_package_manager(dir: &PathBuf) -> Option<String> {
    let lock_file = dir.join("yarn.lock");
    if lock_file.exists() {
        return Some("yarn".to_string());
    }

    let lock_file = dir.join("package-lock.json");
    if lock_file.exists() {
        return Some("npm".to_string());
    }

    let lock_file = dir.join("pnpm-lock.yaml");
    if lock_file.exists() {
        return Some("pnpm".to_string());
    }

    let items = vec!["pnpm", "npm", "yarn"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which package manager do you want to use?")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .ok()?;

    if selection.is_none() {
        return None;
    }

    Some(items[selection.unwrap()].to_string())
}
