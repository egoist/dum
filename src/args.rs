use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
pub struct AppArgs {
    pub script_name: String,
    pub forwared: String,
    pub change_dir: PathBuf,
}

pub fn parse_args(args_vec: &[String]) -> AppArgs {
    let mut args_iter = args_vec.into_iter();

    let mut args = AppArgs {
        script_name: "".to_string(),
        change_dir: PathBuf::from(env::current_dir().as_ref().unwrap()),
        forwared: "".to_string(),
    };

    loop {
        let arg = args_iter.next();
        match arg {
            Some(v) => {
                if v.starts_with("-") {
                    if args.script_name.is_empty() {
                        match v.as_ref() {
                            "-c" => {
                                let dir = match args_iter.next() {
                                    Some(v) => PathBuf::from(v),
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
                        // forwared flags
                        args.forwared.push_str(" ");
                        args.forwared.push_str(&v);
                    }
                } else if args.script_name.is_empty() {
                    args.script_name = match v.as_ref() {
                        "t" => "test".to_string(),
                        "i" => "install".to_string(),
                        _ => v.to_string(),
                    };
                } else {
                    args.forwared.push_str(" ");
                    args.forwared.push_str(&v);
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

fn get_help() -> String {
    format!(
        "\
dum v{}

USAGE:
    dum [OUR_FLAGS] [SCRIPT_NAME] [SCRIPT_ARGS]

COMMANDS:
    add <packages>     Add packages to the current project
    i, install         Install dependencies
    remove <packages>  Remove packages from the current project
    t, test            Run test script in nearest package.json
    [script]           Run scripts in nearest package.json

FLAGS:
    -c <dir>              Change working directory
    -h, --help            Prints help information
",
        get_version()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let args = parse_args(&vec_of_strings!["a", "b", "-c", "-d", "foo", "bar"]);
        assert_eq!(args.script_name, "a".to_string());
        assert_eq!(args.forwared, " b -c -d foo bar".to_string());
    }

    #[test]
    fn test_parse_own_flags() {
        let args = parse_args(&vec_of_strings!["-c", ".", "a"]);
        assert_eq!(args.script_name, "a".to_string());
        assert_eq!(args.change_dir, PathBuf::from("."));
        assert_eq!(args.forwared, "".to_string());
    }
}
