use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let expected_arguments = usize::try_from(3).unwrap();
        if &args.len() < &expected_arguments {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;


    let result = if config.ignore_case {
        search_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents))
    }
}
