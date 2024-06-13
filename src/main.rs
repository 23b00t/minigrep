use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // to collect arguments from std:in
    let args: Vec<String> = env::args().collect();

    // give arguments to struct Config to parse it and return Result
    // to gracefully handle errors
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("An error occured: {}", e);

        process::exit(1);
    }
}

