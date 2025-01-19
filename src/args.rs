extern crate path_absolutize;

use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

use path_absolutize::Absolutize;

#[derive(Debug)]
pub struct AppArgs {
    pub script_name: String,
    pub forwarded: Vec<String>,
    pub change_dir: PathBuf,
    pub command: String,
    pub interactive: bool,
}

pub const COMMANDS_TO_FORWARD: &[&str] = &["install", "i", "add", "remove", "uninstall"];

pub fn parse_args(args_vec: Vec<String>) -> AppArgs {
    let mut args_iter = args_vec.into_iter();

    let mut args = AppArgs {
        script_name: "".to_string(),
        change_dir: PathBuf::from(env::current_dir().as_ref().unwrap()),
        forwarded: vec![],
        command: "".to_string(),
        interactive: false,
    };

    loop {
        let arg = args_iter.next();
        match arg {
            Some(v) => {
                if v == "--" {
                    args.forwarded.extend(args_iter);
                    break;
                }
                if v.starts_with('-') {
                    if args.script_name.is_empty()
                        && (args.command.is_empty() || args.command == "run")
                    {
                        match v.as_ref() {
                            "-c" => {
                                let dir = match args_iter.next() {
                                    Some(v) => PathBuf::from(Path::new(&v).absolutize().unwrap()),
                                    None => {
                                        println!("No directory specified");
                                        exit(1);
                                    }
                                };
                                if !dir.exists() {
                                    println!("Error: directory {} does not exist", dir.display());
                                    std::process::exit(1);
                                }
                                args.change_dir = dir;
                            }
                            "-i" | "--interactive" => {
                                args.interactive = true;
                            }
                            "-h" | "--help" => {
                                print!("{}", get_help());
                                std::process::exit(0);
                            }
                            "-v" | "--version" => {
                                println!("{}", get_version());
                                std::process::exit(0);
                            }
                            _ => {
                                println!("Unknown flag: {}", v);
                                exit(1);
                            }
                        }
                    } else {
                        args.forwarded.push(v);
                    }
                } else if args.command.is_empty()
                    && (COMMANDS_TO_FORWARD.contains(&v.as_str()) || v == "run")
                {
                    args.command = match v.as_ref() {
                        "i" => "install".to_string(),
                        _ => v.to_string(),
                    };
                } else if (args.command.is_empty() || args.command == "run")
                    && args.script_name.is_empty()
                {
                    args.command = "run".to_string();
                    args.script_name = match v.as_ref() {
                        "t" => "test".to_string(),
                        _ => v.to_string(),
                    };
                } else {
                    if args.interactive {
                        eprintln!("You can't pass arguments to interactive mode");
                        exit(1);
                    }
                    args.forwarded.push(v);
                }
            }
            None => break,
        }
    }
    args
}

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn get_help() -> String {
    format!(
        "\
dum v{}

USAGE:
    dum [OUR_FLAGS] [SCRIPT_NAME] [SCRIPT_ARGS]

COMMANDS:
    <script_name>      Run an npm script (like npm run) or a script in node_modules/.bin (like npx)
    run                Show a list of available scripts
    run <script_name>  Run an npm script
    add <packages>     Add packages to the current project
    i, install         Install dependencies
    remove <packages>  Remove packages from the current project
    t, test            Run test script in nearest package.json
    [script]           Run scripts in nearest package.json

FLAGS:
    -c <dir>              Change working directory
    -i, --interactive     Interactive mode
    -h, --help            Prints help information
    -v, --version         Prints version number
",
        get_version()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use path_absolutize::Absolutize;

    macro_rules! vec_of_strings {
        // match a list of expressions separated by comma:
        ($($str:expr),*) => ({
            // create a Vec with this list of expressions,
            // calling String::from on each:
            vec![$(String::from($str),)*] as Vec<String>
        });
    }

    #[test]
    fn test_parse() {
        let args = parse_args(vec_of_strings!["a", "b", "-c", "-d", "foo", "bar"]);
        assert_eq!(args.script_name, "a".to_string());
        assert_eq!(args.forwarded, vec!["b", "-c", "-d", "foo", "bar"]);
    }

    #[test]
    fn test_parse_own_flags() {
        let args = parse_args(vec_of_strings!["-c", ".", "a"]);
        assert_eq!(args.script_name, "a".to_string());
        assert_eq!(args.change_dir, Path::new(".").absolutize().unwrap());
        assert_eq!(args.forwarded, Vec::<&str>::new());
    }
}
