mod args;
mod install;
mod prompt;
mod run;
mod uchecker;

use std::env;
use uchecker::update_check;

fn main() {
    env_logger::init();
    prompt::handle_ctrlc();
    let _ = update_check();
    let args_vec: Vec<String> = env::args().collect();
    let args = args::parse_args(&args_vec[1..]);

    run::run(&args);
}
