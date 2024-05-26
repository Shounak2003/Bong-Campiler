use std::env;
use std::fs;
use std::process::exit;
use std::io;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => run(&contents),
    }
}

fn run(_contents: &String) -> Result<(), String> {
    Err("Not Implemented".to_string())
}

fn run_prompt() -> Result<(), String> {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Could not read line".to_string()),
    }
    println!("You wrote: {}", buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: \n{}", msg); // Line 29
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => (),
            Err(msg) => {
                println!("Error: \n{}", msg); // Line 38
                exit(1);
            }
        }
    }
}

