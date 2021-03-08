use std::env;
use std::process;

use minigrep;

fn main() {
    let config = minigrep::Config::new(env::args(), env::vars().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
