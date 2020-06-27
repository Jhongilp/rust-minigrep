use std::env;
use std::process;
mod minigrep;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Aplication error: {}", e);
        process::exit(1);
    }

    
}

