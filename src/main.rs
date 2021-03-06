extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error on initialization\n{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprintln!("Error running application\n{}", err);
        process::exit(1);
    }
}
