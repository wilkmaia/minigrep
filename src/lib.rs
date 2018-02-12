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

#[cfg(test)]
mod test {
    use super::*;
    use std::process;

    const BIN_NAME: &'static str = "bin_name";
    const FILENAME: &'static str = "__program_run_test_file__";
    const PATTERN: &'static str = "pattern";

    fn get_args() -> Vec<String> {
        let bin_name = BIN_NAME.to_string();
        let pattern = PATTERN.to_string();
        let filename = FILENAME.to_string();

        vec![bin_name, pattern, filename]
    }

    fn touch_file(filename: String) -> Result<process::Output, Box<Error>> {
        let cmd: String;

        if cfg!(target_os = "windows") {
            cmd = format!("fsutil file createnew {}", &filename);
            Ok(process::Command::new("cmd")
                .args(&["/C", cmd.as_str()])
                .output()?)
        } else {
            cmd = format!("touch {}", &filename);
            Ok(process::Command::new("sh")
                .args(&["-c", cmd.as_str()])
                .output()?)
        }
    }

    fn delete_file(filename: String) -> Result<process::Output, Box<Error>> {
        let cmd: String;

        if cfg!(target_os = "windows") {
            cmd = format!("del /F {}", &filename);
            Ok(process::Command::new("cmd")
                .args(&["/C", cmd.as_str()])
                .output()?)
        } else {
            cmd = format!("rm {}", &filename);
            Ok(process::Command::new("sh")
                .args(&["-c", cmd.as_str()])
                .output()?)
        }
    }

    #[test]
    fn create_config() {
        let args = get_args();

        Config::new(&args).unwrap_or_else(|err| {
            panic!(err);
        });
    }

    #[test]
    #[should_panic(expected = "Insufficient parameters")]
    fn create_invalid_config() {
        let args: Vec<String> = vec![];

        Config::new(&args).unwrap_or_else(|err| {
            panic!(err);
        });
    }

    #[test]
    fn base_program_run() {
        let args = get_args();
        let config = Config::new(&args).unwrap();

        let result = touch_file(FILENAME.to_string()).unwrap();
        assert_eq!(result.status.code(), Some(0));

        let result = run(config).unwrap();
        assert_eq!(result, ());

        let result = delete_file(FILENAME.to_string()).unwrap();
        assert_eq!(result.status.code(), Some(0));
    }
}
