use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        println!("Problem Parsing: {e}");

        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
