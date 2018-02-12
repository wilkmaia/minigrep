use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

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
    where T: Iterator<Item = String>,
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

        Ok(Config { filename, pattern, case_sensitive })
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
/// The functions expects a `Config` struct containing:
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

/// Performs a case sensitive search on `text`, looking for `pattern` matches. If a match is found
/// on a line, the whole line is returned.
pub fn search_case_sensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

/// Performs a case insensitive search on `text`, looking for `pattern` matches. If a match is found
/// on a line, the whole line is returned.
pub fn search_case_insensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test;
