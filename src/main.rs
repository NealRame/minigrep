use std::env;
use std::process;

use minigrep;

fn main() {
    let config = minigrep::Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
