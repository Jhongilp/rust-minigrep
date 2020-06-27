use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    // let query = &args[1];
    // let filename = &args[2];
    let (query, filename) = parse_config(&args);
    println!("Searching for: {}", query);
    println!("In file: {}", filename);

    let contens = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n {}", contens);

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}