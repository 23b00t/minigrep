use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let (query, file_path, case) = Self::parse_arguments(args)?;

        let ignore_case = if env::var("IGNORE_CASE").is_ok() {
            true
        } else {
            case
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    fn parse_arguments(args: &[String]) -> Result<(String, String, bool), &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!\nSyntax: minigrep [-i] <search term> <file path>");
        }

        if args.len() == 3 {
            Ok((args[1].clone(), args[2].clone(), false))
        } else if args.len() == 4 {
            if args[1] == "-i" {
                Ok((args[2].clone(), args[3].clone(), true))
            } else {
                Err("Wrong Argument!\nSyntax: minigrep [-i] <search term> <file path>")
            }
        } else {
            Err("Too many arguments!\nSyntax: minigrep [-i] <search term> <file path>")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file to search through to string, ? to unwrap_or_error
    let contents = fs::read_to_string(config.file_path)?;

    // check if search is case (in)sensitive
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

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

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    // 1. iterate over every line of contents
    for line in contents.lines() {
        // 2. check if line as lowercase contains query as lowercase
        // 3. true -> add to results Vec
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "dukt";
        let contents = "\
Rust:
sicher, schnell, produktiv.
Nimm drei.
PRODUKTION";

        assert_eq!(vec!["sicher, schnell, produktiv."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
sicher, schnell, produktiv.
Nimm drei.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], search_case_insensitive(query, contents)
        );
    }
}
