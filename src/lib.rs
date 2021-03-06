extern crate regex; 

use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use regex::RegexBuilder;

/// Holds needed configuration information for the tool.
///
/// - `filename`: The file in which the search will be conducted
/// - `pattern`: The pattern the tool will search
/// - `case_sensitive`: Specifies if the search should be case sensitive or not
pub struct Config {
    filename: String,
    pattern: String,
    case_sensitive: bool,
}

impl Config {
    /// Creates a new Config struct based on user input.
    /// `new` expects as parameter an iterator whose `Item` is a `String` struct.
    ///
    /// # Examples
    ///
    /// Consider the following usage:
    /// `./minigrep pattern filename`
    ///
    /// The following should not fail
    ///
    /// ```
    /// let bin_name = String::from("minigrep");
    /// let pattern = String::from("pattern");
    /// let filename = String::from("filename");
    /// let mut args = vec![bin_name, pattern, filename].into_iter();
    ///
    /// let config = minigrep::Config::new(args).unwrap();
    ///
    /// assert_eq!(config.pattern(), "pattern");
    /// assert_eq!(config.filename(), "filename");
    /// assert_eq!(config.case_sensitive(), &true);
    /// ```
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
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

        Ok(Config {
            filename,
            pattern,
            case_sensitive,
        })
    }

    /// Returns the Config's filename value
    ///
    /// # Examples
    ///
    /// ```
    /// let bin_name = String::from("minigrep");
    /// let pattern = String::from("pattern");
    /// let filename = String::from("filename");
    /// let mut args = vec![bin_name, pattern, filename].into_iter();
    ///
    /// let config = minigrep::Config::new(args).unwrap();
    ///
    /// assert_eq!(config.filename(), "filename");
    /// ```
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Returns the Config's search pattern value
    ///
    /// # Examples
    ///
    /// ```
    /// let bin_name = String::from("minigrep");
    /// let pattern = String::from("pattern");
    /// let filename = String::from("filename");
    /// let mut args = vec![bin_name, pattern, filename].into_iter();
    ///
    /// let config = minigrep::Config::new(args).unwrap();
    ///
    /// assert_eq!(config.pattern(), "pattern");
    /// ```
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Returns the Config's case sensitive flag value
    ///
    /// # Examples
    ///
    /// ```
    /// let bin_name = String::from("minigrep");
    /// let pattern = String::from("pattern");
    /// let filename = String::from("filename");
    /// let mut args = vec![bin_name, pattern, filename].into_iter();
    ///
    /// let config = minigrep::Config::new(args).unwrap();
    ///
    /// assert_eq!(config.case_sensitive(), &true);
    /// ```
    pub fn case_sensitive(&self) -> &bool {
        &self.case_sensitive
    }
}

/// Runs the library logic.
/// The functions expects a reference to a `Config` struct containing:
///
/// - `filename`: The file in which the search will be conducted
/// - `pattern`: The pattern the tool will search
/// - `case_sensitive`: Specifies if the search should be case sensitive or not
///
/// If the execution runs without any problem, the function returns `()`. If not, a
/// `Box<std::error::Error>` struct is returned.
///
/// # Errors
///
/// The function returns errors on the following situations:
///
/// - `filename` doesn't exist
/// - for any reason `filename` could not be read (maybe no read permission)
pub fn run(config: &Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let matches = search_regex(config.pattern(), &contents, &config.case_sensitive);

    for item in matches {
        println!("{}", item);
    }

    Ok(())
}

/// Performs a regular expression search on `text`, looking for `pattern` matches. Case sensitivity is
/// controlled by the `case_sensitive` boolean flag. If a match is found on a line, 
/// the whole line is returned.
pub fn search_regex<'a>(pattern: &str, text: &'a str, case_sensitive: &bool) -> Vec<&'a str> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(!case_sensitive)
        .build();

    text.lines()
        .filter(|line| {
            match re {
                Ok(ref regex) => regex.is_match(line),
                Err(_) => line.contains(pattern),
            }
        }) 
        .collect()
}

#[cfg(test)]
mod test;
