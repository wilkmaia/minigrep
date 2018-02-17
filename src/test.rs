use super::*;
use std::process;

const BIN_NAME: &'static str = "bin_name";
const FILENAME: &'static str = "__program_run_test_file__";
const PATTERN: &'static str = "pattern";

fn args() -> Vec<String> {
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
    let args = args().into_iter();

    let config = Config::new(args).unwrap_or_else(|err| {
        panic!(err);
    });

    assert_eq!(config.filename(), FILENAME);
    assert_eq!(config.pattern(), PATTERN);
    assert_eq!(config.case_sensitive(), &true);
}

#[test]
#[should_panic(expected = "Missing search pattern")]
fn create_invalid_config() {
    let args = vec![].into_iter();

    Config::new(args).unwrap_or_else(|err| {
        panic!(err);
    });
}

#[test]
fn base_program_run() {
    let args = args().into_iter();
    let config = Config::new(args).unwrap();

    let result = touch_file(FILENAME.to_string()).unwrap();
    assert_eq!(result.status.code(), Some(0));

    let result = run(&config).unwrap();
    assert_eq!(result, ());

    let result = delete_file(FILENAME.to_string()).unwrap();
    assert_eq!(result.status.code(), Some(0));
}

#[test]
fn case_sensitive_lower_to_lower() {
    let pattern = "duct";
    let text = "\
Rust:
Safe, fast, productive.
Pick three.";

    assert_eq!(
        search_regex(pattern, text, &true),
        vec!["Safe, fast, productive."]
    );
}

#[test]
fn case_sensitive_upper_to_upper() {
    let pattern = "DUCT";
    let text = "\
RUST:
SAFE, FAST, PRODUCTIVE.
PICK THREE.";

    assert_eq!(
        search_regex(pattern, text, &true),
        vec!["SAFE, FAST, PRODUCTIVE."]
    );
}

#[test]
fn case_sensitive_upper_to_lower() {
    let pattern = "DUCT";
    let text = "\
Rust:
Safe, fast, productive.
Pick three.";

    assert_eq!(
        search_regex(pattern, text, &true).len(),
        0
    );
}

#[test]
fn case_insensitive() {
    let pattern = "DUCT";
    let text = "\
Rust:
Safe, fast, productive.
Pick three.";

    assert_eq!(
        search_regex(pattern, text, &false),
        vec!["Safe, fast, productive."]
    );
}

#[test]
fn regex() {
    let pattern = "fast.*tive";
    let text = "\
Rust:
Safe, fast, productive.
Pick three.";
    
    assert_eq!(
        search_regex(pattern, text, &false),
        vec!["Safe, fast, productive."]
        )
}

#[test]
fn invalid_regex_fallback_to_string() {
    let pattern = "!f!o---b\\ar|";
    let text = "\
!f!o---b\\ar|
Rust:
Safe, fast, productive.
Pick three.";
    
    assert_eq!(
        search_regex(pattern, text, &false),
        vec!["!f!o---b\\ar|"]
        )
}
