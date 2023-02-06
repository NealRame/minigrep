use std::env;
use std::process;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config =
        minigrep::Config::build(&args)
            .unwrap_or_else(|err| {
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            });

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
