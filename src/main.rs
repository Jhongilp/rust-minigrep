use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Aplication error: {}", e);
        process::exit(1);
    }

    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let contens =
        fs::read_to_string(config.filename)?;

    println!("With text:\n {}", contens);

    Ok(())
}