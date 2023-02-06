use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        std::process::exit(1);
    }
}

// Logic
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{contents}");

    Ok(())
}

// Config
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
