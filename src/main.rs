use serde_json::Value;
use std::env;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::process::{exit, Command};

struct AppArgs {
    script_name: String,
    remaing_args: Vec<OsString>,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let contents = read_to_string("package.json").expect("failed to read package.json");
    let v: Value = serde_json::from_str(&contents).expect("failed to parse package.json");

    println!("> {}", args.script_name);

    let result = v
        .get("scripts")
        .and_then(|scripts| scripts.get(&args.script_name))
        .and_then(|script| script.as_str())
        .map(|script| {
            println!("> {}", script);
            let remaining = args_to_string(&args.remaing_args);

            let (sh, sh_flag) = if cfg!(target_os = "windows") {
                ("cmd", "/C")
            } else {
                ("sh", "-c")
            };
            let status = Command::new(sh)
                .arg(sh_flag)
                .arg([script, &remaining].join(" "))
                .env("PATH", get_path_env())
                .status()
                .expect("failed to execute script");

            exit(status.code().unwrap_or(1));
        });

    if result.is_none() {
        eprintln!("Error: script not found.");
        std::process::exit(1);
    }
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", get_help());
        std::process::exit(0);
    }
    // All our flags should be parsed before parsing the script name
    let mut script_name: String = "".to_string();

    loop {
        let arg = pargs.opt_free_from_str::<String>()?;

        match arg {
            Some(v) => {
                if v.starts_with("-") {
                    continue;
                } else {
                    script_name = v;
                    break;
                }
            }
            None => break,
        }
    }

    if script_name.is_empty() {
        return Err(pico_args::Error::ArgumentParsingFailed {
            cause: "No script name provided".to_string(),
        });
    }

    let args = AppArgs {
        script_name,
        remaing_args: pargs.finish(),
    };

    Ok(args)
}

fn args_to_string(args: &Vec<OsString>) -> String {
    let mut s = String::new();
    for arg in args {
        s.push_str(&arg.to_string_lossy());
        s.push(' ');
    }
    s
}

fn get_path_env() -> String {
    return [env!("PATH"), &"./node_modules/.bin".to_string()].join(":");
}

fn get_help() -> String {
    format!(
        "\
dum v{}

USAGE:
    dum [OUR_FLAGS] [SCRIPT_NAME] [SCRIPT_ARGS]

FLAGS:
    -h, --help            Prints help information
",
        env!("CARGO_PKG_VERSION")
    )
}
