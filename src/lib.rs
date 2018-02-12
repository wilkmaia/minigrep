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
