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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("Missing search pattern"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Missing filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { filename, pattern, case_sensitive })
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let matches = if config.case_sensitive {
        search_case_sensitive(config.pattern(), &contents)
    } else {
        search_case_insensitive(config.pattern(), &contents)
    };

    for item in matches {
        println!("{}", item);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test;
