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

#[test]
fn one_result() {
    let pattern = "duct";
    let text = "\
Rust:
Safe, fast, productive.
Pick three.";

    assert_eq!(
        search(pattern, text),
        vec!["Safe, fast, productive."]
    );
}