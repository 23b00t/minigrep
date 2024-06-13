use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // expected arguments: 2
        if args.len() < 3 {
            return Err("Not enough arguments!\nSyntax: minigrep <search term> <file path>");
        }
        // clone to avoid lifetime handeling -> improve later!!
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file to search through to string, ? to unwrap_or_error
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // 1. iterate over every line of contents
    for line in contents.lines() {
        // 2. check if line contains query
        // 3. true -> add to results Vec
        if line.contains(query) {
            results.push(line);
        }
    }
    // 4. return results Vec
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "dukt";
        let contents = "\
Rust:
sicher, schnell, produktiv.
Nimm drei.";

        assert_eq!(vec!["sicher, schnell, produktiv."], search(query, contents));
    }
}
