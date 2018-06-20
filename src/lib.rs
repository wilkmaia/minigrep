extern crate regex;

use regex::Captures;
use regex::RegexBuilder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

static ANSI_REVERSE: &'static str = "\x1b[7m";
static ANSI_RESET: &'static str = "\x1b[0m";

/// Holds needed configuration information for the tool.
///
/// - `filename`: The file in which the search will be conducted
/// - `pattern`: The pattern the tool will search
/// - `case_sensitive`: Specifies if the search should be case sensitive or not
/// - `highlight_match`: Specifies if the match should be highlighted or not
pub struct Config {
    filename: String,
    pattern: String,
    case_sensitive: bool,
    highlight_match: bool,
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
    /// let case_sensitive = true;
    /// let highlight_match = true;
    /// let mut args = vec![bin_name, pattern, filename].into_iter();
    ///
    /// let config = minigrep::Config::new(args).unwrap();
    ///
    /// assert_eq!(config.pattern(), "pattern");
    /// assert_eq!(config.filename(), "filename");
    /// assert_eq!(config.case_sensitive(), &true);
    /// assert_eq!(config.highlight_match(), &true);
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

        // TODO implement case sensitivity match hightlight command line flags

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        let highlight_match = true;

        Ok(Config {
            filename,
            pattern,
            case_sensitive,
            highlight_match,
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

    /// Returns the Config's hightlight match flag value
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
    pub fn highlight_match(&self) -> &bool {
        &self.highlight_match
    }
}

/// Runs the library logic.
/// The functions expects a reference to a `Config` struct containing:
///
/// - `filename`: The file in which the search will be conducted
/// - `pattern`: The pattern the tool will search
/// - `case_sensitive`: Specifies if the search should be case sensitive or not
/// - `highlight_match`: Specifies if the match should be highlighted or not
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

    let matches = search_regex(config.pattern(), &contents, config.case_sensitive, true);

    for item in matches {
        println!("{}", item);
    }

    Ok(())
}

/// Performs a regular expression search on `text`, looking for `pattern` matches.
/// In the case `pattern` isn't a valid regular expressions, match against it as a simple pattern.
/// If a match is found on a line, the whole line is returned.
/// Case sensitivity is controlled by the `case_sensitive` boolean flag.
/// `highlight_match` boolean flag is used to return a highlighted.

pub fn search_regex(
    pattern: &str,
    text: &str,
    case_sensitive: bool,
    highlight_match: bool,
) -> Vec<String> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(!case_sensitive)
        .build();

    text.lines().fold(vec![], |mut acc, line| {
        match re {
            Ok(ref regex) => {
                if regex.is_match(line) {
                    if highlight_match {
                        let result = regex.replace_all(line, |caps: &Captures| {
                            format!("{}{}{}", ANSI_REVERSE, &caps[0], ANSI_RESET)
                        });

                        acc.push(result.into_owned());
                    } else {
                        acc.push(String::from(line));
                    }
                }
            }
            Err(_) => {
                if line.contains(pattern) {
                    if highlight_match {
                        let formated_string = format!("{}{}{}", ANSI_REVERSE, pattern, ANSI_RESET);
                        let result = line.replace(pattern, formated_string.as_str());
                        acc.push(result);
                    } else {
                        acc.push(String::from(line));
                    }
                }
            }
        };
        acc
    })
}

#[cfg(test)]
mod test;
