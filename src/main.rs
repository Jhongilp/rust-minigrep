use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Config::new(&args);
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let contens =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n {}", contens);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
