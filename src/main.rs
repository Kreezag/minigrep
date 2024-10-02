use std::{env, process};
use std::fs;
use std::convert::TryFrom;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem Parsing: {e}");

        process::exit(1);
    });

    println!("Searching query {}", config.query);
    println!("Searching path {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
        .expect("Should have beet able to read the file");

    println!("Content:\n {content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let expected_arguments = usize::try_from(3).unwrap();
        if &args.len() < &expected_arguments {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}