use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    filename: String,
    pattern: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient parameters");
        }

        let pattern = args[1].to_string();
        let filename = args[2].to_string();

        Ok(Config { filename, pattern })
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_pattern(&self) -> &String {
        &self.pattern
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.get_filename())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
}

pub fn search<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in text.lines() {
        if line.contains(pattern) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test;
