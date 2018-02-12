extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    println!("Filename = {}", config.get_filename());
    println!("Pattern = {}", config.get_pattern());
}
