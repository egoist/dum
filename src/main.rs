use dum::{dum, parse_args};
use std::env;

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    let args = parse_args(&args_vec[1..]);

    dum(&args);
}
