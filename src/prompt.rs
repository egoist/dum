use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn select(message: &str, script_names: Vec<&str>) -> Option<String> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .items(&script_names)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .ok()?;

    Term::stderr().show_cursor().expect("failed to show cursor");

    if selection.is_none() {
        return None;
    }

    Some(script_names[selection.unwrap()].to_string())
}

pub fn input(message: &str) -> String {
    let input = Input::<String>::new()
        .with_prompt(message)
        .allow_empty(true)
        .with_initial_text("")
        .interact_text_on(&Term::stderr())
        .ok();

    Term::stderr().show_cursor().expect("failed to show cursor");

    input.expect("should have input")
}
