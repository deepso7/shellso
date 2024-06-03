#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    while stdin.read_line(&mut input).is_ok() {
        let token = tokenize(input.trim());

        match token[..] {
            ["exit", code] => exit(match code.parse::<i32>() {
                Ok(num) => num,
                Err(_e) => {
                    println!("enter valid integer");
                    1
                }
            }),

            ["echo", ..] => println!("{}", token[1..].join(" ")),

            _ => println!("{}: command not found", input.trim()),
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
