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
fn get_path_env(bin_dirs: Vec<PathBuf>) -> String {
    let mut path = env::var("PATH").unwrap_or_default();
    for dir in bin_dirs {
        path.push_str(":");
        path.push_str(dir.to_str().unwrap());
    }
    path
}

// A function to find the closest file
// Starting from current directory
// Recusively until it finds the file or reach root directory
fn find_closest_files(_current_dir: &PathBuf, name: &str, stop_on_first: bool) -> Vec<PathBuf> {
    let mut closest_file: Vec<PathBuf> = Vec::new();
    let mut current_dir = _current_dir.clone();

    loop {
        let path = current_dir.join(name);
        if path.exists() {
            closest_file.push(path);
            if stop_on_first {
                break;
            }
        }
        match current_dir.parent() {
            Some(p) => current_dir = p.to_path_buf(),
            None => break,
        }
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

fn resolve_bin_path(bin_name: &str, dirs: &Vec<PathBuf>) -> Option<PathBuf> {
    for dir in dirs {
        let path = dir.join(bin_name);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub fn dum(app_args: &args::AppArgs) {
    let pkg_paths = find_closest_files(&app_args.change_dir, "package.json", true);
    let pkg_path = if pkg_paths.is_empty() {
        println!("No package.json found");
        exit(1);
    } else {
        pkg_paths[0].clone()
    };

    // The current_dir to execute npm scripts
    let execute_dir = PathBuf::from(pkg_path.parent().unwrap());

    let node_modules_dirs = find_closest_files(&app_args.change_dir, "node_modules", false);
    let bin_dirs = node_modules_dirs
        .iter()
        .map(|dir| dir.join(".bin"))
        .collect::<Vec<PathBuf>>();

    let contents = read_to_string(pkg_path).expect("failed to read package.json");
    let v: Value = serde_json::from_str(&contents).expect("failed to parse package.json");

    if app_args.command == "run" && app_args.script_name.is_empty() {
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

    if app_args.script_name.is_empty() {
        println!("No script name specified.\n");
        println!("{}", args::get_help());
        return;
    }

    // Run npm install if the script_name is "install"
    if ["install", "add", "remove"].contains(&app_args.script_name.as_str()) {
        let pm = install::guess_package_manager(&execute_dir);

        if pm.is_none() {
            eprintln!("No package manager found.");
            exit(1);
        }

        run_command(
            &[&pm.unwrap(), &app_args.script_name, &app_args.forwared],
            &RunOptions {
                current_dir: execute_dir,
                envs: HashMap::new(),
            },
        );
        return;
    }

    let npm_script = v.get("scripts").and_then(|scripts| {
        scripts.as_object().and_then(|scripts| {
            scripts
                .get(app_args.script_name.as_str())
                .and_then(|script| {
                    let script = script.as_str().map(|script| script.to_string());
                    Some(script.unwrap_or_default())
                })
        })
    });

    if npm_script.is_some() {
        let script = npm_script.unwrap();
        println!("> {}", app_args.script_name);
        println!("> {}{}", script, app_args.forwared);
        let envs = HashMap::from([("PATH".to_string(), get_path_env(bin_dirs))]);
        run_command(
            &[&script, &app_args.forwared],
            &RunOptions {
                current_dir: execute_dir,
                envs,
            },
        );
        return;
    }

    let resolved_bin = resolve_bin_path(app_args.script_name.as_str(), &bin_dirs);
    if resolved_bin.is_some() {
        let bin_path = resolved_bin.unwrap();
        println!("> {}", app_args.script_name);
        println!("> {}{}", bin_path.to_str().unwrap(), app_args.forwared);
        let envs = HashMap::from([("PATH".to_string(), get_path_env(bin_dirs))]);
        run_command(
            &[bin_path.to_str().unwrap(), &app_args.forwared],
            &RunOptions {
                current_dir: execute_dir,
                envs,
            },
        );
        return;
    }

    println!("No script found.");
    println!("To see a list of scripts, run `dum run`");
    exit(1);
}
