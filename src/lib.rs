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
    // read file to search through to string
    let contents = fs::read_to_string(config.file_path)?;

    println!("text:\n{contents}");
    Ok(())
}
