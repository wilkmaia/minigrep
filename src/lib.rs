use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    filename: String,
    pattern: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient parameters");
        }

        let pattern = args[1].to_string();
        let filename = args[2].to_string();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { filename, pattern, case_sensitive })
    }

    pub fn get_filename(&self) -> &str {
        &self.filename.as_str()
    }

    pub fn get_pattern(&self) -> &str {
        &self.pattern.as_str()
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.get_filename())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let matches = search_case_sensitive(config.get_pattern(), &contents);
    for item in matches {
        println!("{}", item);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in text.lines() {
        if line.contains(pattern) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut result = vec![];

    for line in text.lines() {
        if line.to_lowercase().contains(&pattern) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test;
