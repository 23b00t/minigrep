use std::env;
use std::fs;
use std::process;

fn main() {
    // to collect arguments from std:in
    let args: Vec<String> = env::args().collect();

    // give arguments to struct Config to parse it and return Result
    // to gracefully handle errors
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("query: {}, file path: {}", config.query, config.file_path);

    // read file to search through to string
    let contents = fs::read_to_string(config.file_path)
        .expect("something went wrong reading the file");
    // println!("text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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
