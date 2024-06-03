#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        match input.trim().to_lowercase().as_str() {
            "exit 0" => exit(0),

            _ => println!("{}: command not found", input.strip_suffix("\n").unwrap()),
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
