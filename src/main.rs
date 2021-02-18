use std::{env, process};
use std::{error::Error, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} ...", config.query);

    println!("In file {}", config.filename);

    // doesn't return a value that we want to unwrap
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
// Box<dyn Error> a type that implements the error trait
// ? will return the error value from the current function for the caller to handle.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // idiomatic way to say we call run for its side effects only
    // not the return value
    Ok(())
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
