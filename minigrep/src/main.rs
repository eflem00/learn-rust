use std::env;
use std::process;

use minigrep_evan;

fn main() {
    let config =
        minigrep_evan::Config::new(env::args(), env::vars().collect()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    minigrep_evan::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
