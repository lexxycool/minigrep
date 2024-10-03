use std::{env, fs, process};
use config::Config;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args)
    .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file: {}", config.config_query(), config.config_file_path());

    let contents = fs::read_to_string(config.config_file_path())
    .expect("should be able to read the file");

    println!("With text:\n {contents}");
    
}

