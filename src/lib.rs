mod args;
mod install;

// Re-export
pub use crate::args::parse_args;

use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::{exit, Command};

// Get PATH env and join it with bin_dir
fn get_path_env(bin_dir: &str) -> String {
    let mut path = env::var("PATH").unwrap_or_default();
    path.push_str(":");
    path.push_str(bin_dir);
    path
}

// A function to find the closest file
// Starting from current directory
// Recusively until it finds the file or reach root directory `/`
fn find_closest_file(_current_dir: &PathBuf, name: &str) -> Option<PathBuf> {
    let mut closest_file = None;
    let stop_dir = "/".to_string();
    let mut current_dir = _current_dir.clone();

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

pub fn dum(args: &args::AppArgs) {
    let pkg_path = find_closest_file(&args.change_dir, "package.json").expect("no package.json");
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

    // Run npm install if the script_name is "install"
    if ["install", "add", "remove"].contains(&args.script_name.as_str()) {
        let pm = install::guess_package_manager(&execute_dir);

        if pm.is_none() {
            eprintln!("No package manager found.");
            exit(1);
        }

        run_command(
            &[&pm.unwrap(), &args.script_name, &args.forwared],
            &RunOptions {
                current_dir: execute_dir,
                envs: HashMap::new(),
            },
        );
        return;
    }

    let result = v
        .get("scripts")
        .and_then(|scripts| match scripts.get(&args.script_name) {
            Some(script) => {
                println!("> {}", args.script_name);
                println!("> {}{}", script.as_str().unwrap(), args.forwared);
                script.as_str().map(|script| script.to_string())
            }
            None => {
                let bin_file = bin_dir.join(&args.script_name);
                if bin_file.exists() {
                    println!("> {}", bin_file.display());
                    Some(bin_file.to_string_lossy().to_string())
                } else {
                    None
                }
            }
        })
        .map(|script| {
            let envs =
                HashMap::from([("PATH".to_string(), get_path_env(&bin_dir.to_str().unwrap()))]);

            run_command(
                &[&script, &args.forwared],
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
