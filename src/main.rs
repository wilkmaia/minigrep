extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Error on initialization\n{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        println!("Error running application\n{}", err);
        process::exit(1);
    }
}
