use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
// Box<dyn Error> a type that implements the error trait
// ? will return the error value from the current function for the caller to handle.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // idiomatic way to say we call run for its side effects only
    // not the return value
    Ok(())
}
// In this case, we indicate that the returned vector should contain
// string slices that reference slices of the argument contents (rather than the argument query).
// In other words, we tell Rust that the data returned by the search function
// will live as long as the data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
