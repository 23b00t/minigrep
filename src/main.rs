use std::env;
use std::fs;

fn main() {
    // to collect arguments from std:in
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    // println!("query: {}, file path: {}", query, file_path);
    let contents = fs::read_to_string(file_path)
        .expect("something went wrong reading the file");
    // println!("text:\n{contents}");
}
