mod install;

use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::{exit, Command};

struct AppArgs {
    script_name: String,
    remaining: Vec<OsString>,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let pkg_path = find_closest_file("package.json").expect("no package.json");
    // The current_dir to execute npm scripts
    let execute_dir = PathBuf::from(pkg_path.parent().unwrap());
    // bin_dir is the dirname of pkg_data followed by node_modules/.bin
    let bin_dir = PathBuf::from(execute_dir.join("node_modules").join(".bin"));

    let contents = read_to_string(pkg_path).expect("failed to read package.json");
    let v: Value = serde_json::from_str(&contents).expect("failed to parse package.json");

    if args.script_name.is_empty() {
        if let Some(scripts) = v["scripts"].as_object() {
            println!("\nAvailable scripts:\n");
            for (name, value) in scripts {
                println!("{}", name);
                println!("  {}", value);
            }
        } else {
            println!("No scripts found.");
        }
        return;
    }

    let remaining = args_to_string(&args.remaining);

    // Run npm install if the script_name is "install"
    if args.script_name == "install" || args.script_name == "add" {
        let pm = install::guess_package_manager(&execute_dir);

        if pm.is_none() {
            eprintln!("No package manager found.");
            exit(1);
        }

        run_command(
            &[&pm.unwrap(), &args.script_name, &remaining],
            &RunOptions {
                current_dir: execute_dir,
                envs: HashMap::new(),
            },
        );
        return;
    }

    println!("> {}", args.script_name);

    let result = v
        .get("scripts")
        .and_then(|scripts| scripts.get(&args.script_name))
        .and_then(|script| script.as_str())
        .map(|script| {
            println!("> {}", script);

            let envs =
                HashMap::from([("PATH".to_string(), get_path_env(&bin_dir.to_str().unwrap()))]);

            run_command(
                &[script, &remaining],
                &RunOptions {
                    current_dir: execute_dir,
                    envs,
                },
            );
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

    // Alias t to test
    if script_name == "t" {
        script_name = "test".to_string();
    }

    // Alias i to install
    if script_name == "i" {
        script_name = "install".to_string();
    }

    let args = AppArgs {
        script_name,
        remaining: pargs.finish(),
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

// Get PATH env and join it with bin_dir
fn get_path_env(bin_dir: &str) -> String {
    let mut path = env::var("PATH").unwrap_or_default();
    path.push_str(":");
    path.push_str(bin_dir);
    path
}

fn get_help() -> String {
    format!(
        "\
dum v{}

USAGE:
    dum [OUR_FLAGS] [SCRIPT_NAME] [SCRIPT_ARGS]

COMMANDS:
    add <packages>  Add packages to the current project
    i, install      Install dependencies
    t, test         Run test script in nearest package.json
    [script]        Run scripts in nearest package.json

FLAGS:
    -h, --help            Prints help information
",
        env!("CARGO_PKG_VERSION")
    )
}

// A function to find the closest file
// Starting from current directory
// Recusively until it finds the file or reach root directory `/`
fn find_closest_file(name: &str) -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();
    let mut closest_file = None;
    let stop_dir = "/".to_string();

    loop {
        let path = current_dir.join(name);
        if path.exists() {
            closest_file = Some(PathBuf::from(path.to_str().unwrap()));
            break;
        }

        if current_dir.to_str().unwrap() == stop_dir {
            break;
        }

        current_dir = current_dir.parent().unwrap().to_path_buf();
    }

    closest_file
}

struct RunOptions {
    envs: HashMap<String, String>,
    current_dir: PathBuf,
}

fn run_command(args: &[&str], options: &RunOptions) {
    let (sh, sh_flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let status = Command::new(sh)
        .arg(sh_flag)
        .arg(args.join(" "))
        .envs(&options.envs)
        .current_dir(&options.current_dir)
        .status()
        .expect("failed to execute the command");

    exit(status.code().unwrap_or(1));
}
