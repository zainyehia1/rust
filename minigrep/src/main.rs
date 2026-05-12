use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};
use colored::Colorize;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // Writing to stderr
        process::exit(1);
    });
    
    println!("Searching for {} in file {}:", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}"); // Writing to stderr
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let search_results = if config.ignore_case {
        &search_case_insensitive(&config.query, &contents)
    } else {
        &search(&config.query, &contents)
    };
    
    if search_results.is_empty() {
        println!("{}", format!("'{}' was not found in {}.", config.query, config.file_path).red());
    } else {
        if search_results.len() == 1 {
            println!("{}", format!("Found 1 match.").blue());
        } else {
            println!("{}", format!("Found {} matches.", search_results.len()).blue());
        }
        
        for line in search_results {
            println!("{}", line.blue());
        }
    }
    
    Ok(())
}
