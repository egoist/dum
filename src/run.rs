use crate::{args, install, prompt};

use ansi_term::{
    Color::{Purple, Red},
    Style,
};
use log::debug;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

// Get PATH env and join it with bin_dir
fn get_path_env(bin_dirs: Vec<PathBuf>) -> String {
    let path = PathBuf::from(env::var("PATH").unwrap_or_default());
    env::join_paths(
        bin_dirs
            .iter()
            .chain(env::split_paths(&path).collect::<Vec<PathBuf>>().iter()),
    )
    .ok()
    .unwrap()
    .into_string()
    .unwrap()
}

// A function to find the closest file
// Starting from current directory
// Recursively until it finds the file or reach root directory
fn find_closest_files(_current_dir: &Path, name: &str, stop_on_first: bool) -> Vec<PathBuf> {
    let mut closest_file: Vec<PathBuf> = Vec::new();
    let mut current_dir = Path::new(_current_dir);
    loop {
        let path = current_dir.join(name);

        if path.exists() {
            closest_file.push(path);
            if stop_on_first {
                break;
            }
        }
        match current_dir.parent() {
            Some(p) => current_dir = p,
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

    // assign the value of options.current_dir to current_dir
    let status = Command::new(sh)
        .arg(sh_flag)
        .arg(args.join(" "))
        .envs(&options.envs)
        .current_dir(&options.current_dir)
        .status()
        .expect("failed to execute the command");

    exit(status.code().unwrap_or(1));
}

fn resolve_bin_path(bin_name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in dirs {
        let path = dir.join(bin_name);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

// Print the script name / script
fn print_script_info(script_name: &str, script: &str, forwarded: &str) {
    println!(
        "{} {}",
        Purple.dimmed().paint("$"),
        Style::new().bold().dimmed().paint(script_name)
    );
    println!(
        "{} {}{}",
        Purple.dimmed().paint("$"),
        Style::new().bold().dimmed().paint(script),
        Style::new().bold().dimmed().paint(forwarded),
    );
}

pub fn run(app_args: &args::AppArgs) {
    if args::COMMANDS_TO_FORWARD.contains(&app_args.command.as_str()) {
        debug!("Running command {}", app_args.command);
        let pm = install::guess_package_manager(&app_args.change_dir);

        if pm.is_none() {
            eprintln!("Aborted.");
            exit(1);
        }

        run_command(
            &[&pm.unwrap().to_string(), &app_args.command, &app_args.forwarded],
            &RunOptions {
                current_dir: app_args.change_dir.clone(),
                envs: HashMap::new(),
            },
        );
        return;
    }

    debug!("change dir to {}", app_args.change_dir.display());
    let pkg_paths = find_closest_files(&app_args.change_dir, "package.json", true);
    let pkg_path = if pkg_paths.is_empty() {
        eprintln!("No package.json found");
        exit(1);
    } else {
        pkg_paths[0].clone()
    };

    debug!("Found package.json at {}", pkg_path.display());
    // The current_dir to execute npm scripts
    let execute_dir = PathBuf::from(pkg_path.parent().unwrap());
    debug!("execute_dir: {:?}", execute_dir);

    let node_modules_dirs = find_closest_files(&app_args.change_dir, "node_modules", false);
    let bin_dirs = node_modules_dirs
        .iter()
        .map(|dir| dir.join(".bin"))
        .collect::<Vec<PathBuf>>();

    let contents = read_to_string(pkg_path).expect("failed to read package.json");
    let v: Value = serde_json::from_str(&contents).expect("failed to parse package.json");

    let scripts = v["scripts"].as_object();
    let mut script_name = app_args.script_name.clone();
    let mut forwarded = app_args.forwarded.clone();

    if !app_args.interactive && app_args.command == "run" && script_name.is_empty() {
        match scripts {
            Some(scripts) => {
                println!("\n{}:\n", Style::new().bold().paint("Available scripts"));
                for (name, value) in scripts {
                    println!("{}", Purple.paint(name));
                    println!("  {}", value.as_str().unwrap());
                }
                return;
            }
            None => {
                eprintln!("No scripts found");
                exit(1);
            }
        }
    }

    if !script_name.is_empty() && app_args.interactive {
        eprintln!("You can't specify script name in interactive mode");
        exit(1);
    }

    if script_name.is_empty() {
        if !app_args.interactive {
            println!("No script name specified.\n");
            println!("{}", args::get_help());
            return;
        }

        if scripts.is_none() {
            eprintln!("No scripts found in package.json");
            exit(1);
        }

        // Choose an script interactively
        // Convert keys of scripts to a vector of &str
        let names_vec = scripts
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>();
        script_name = match prompt::select("Select an npm script to run", names_vec) {
            Some(name) => name,
            None => {
                println!("No script selected.");
                return;
            }
        };
        forwarded = " ".to_string();
        forwarded.push_str(
            match &prompt::input("Enter arguments to pass to the script") {
                Some(args) => args,
                None => {
                    println!("Aborted.");
                    return;
                }
            },
        );
    }

    let npm_script = scripts
        .and_then(|s| s.get(script_name.as_str()))
        .map(|script| {
            let script = script.as_str().map(|script| script.to_string());
            script.unwrap_or_default()
        });
    if let Some(script) = npm_script {
        print_script_info(&script_name, &script, &forwarded);
        let envs = HashMap::from([("PATH".to_string(), get_path_env(bin_dirs))]);
        run_command(
            &[&script, &forwarded],
            &RunOptions {
                current_dir: execute_dir,
                envs,
            },
        );
        return;
    }
    let resolved_bin = resolve_bin_path(script_name.as_str(), &bin_dirs);
    if let Some(bin_path) = resolved_bin {
        print_script_info(&script_name, bin_path.to_str().unwrap(), &forwarded);
        let envs = HashMap::from([("PATH".to_string(), get_path_env(bin_dirs))]);
        run_command(
            &[bin_path.to_str().unwrap(), &forwarded],
            &RunOptions {
                current_dir: execute_dir,
                envs,
            },
        );
        return;
    }

    // TODO: a custom logger module
    println!("{}", Red.normal().paint("No script found."));
    println!("To see a list of scripts, run `dum run`");
    exit(1);
}
