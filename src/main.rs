use rparse::Config;
use std::{env, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // If no args are passed, add a dummy arg
    if args.len() < 2 {
        args.push(String::from("testbin.exe"));
    }

    // Parse args
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    // Run program
    if let Err(e) = rparse::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
