use std::env;
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
        let tokens = tokenize(input.trim());

        match tokens[0] {
            "exit" => exit(match tokens[1].parse::<i32>() {
                Ok(num) => num,
                Err(_e) => {
                    println!("enter valid integer");
                    1
                }
            }),

            "echo" => println!("{}", tokens[1..].join(" ")),

            "type" => {
                let path_env: Result<String, env::VarError> = env::var("PATH");

                let found_path = path_env
                    .unwrap()
                    .split(":")
                    .map(|path| format!("{}/{}", path, tokens[1]))
                    .find(|path| std::fs::metadata(path).is_ok());

                match tokens[1] {
                    "echo" => println!("{} is a shell builtin", tokens[1].trim()),
                    "exit" => println!("{} is a shell builtin", tokens[1].trim()),
                    "type" => println!("{} is a shell builtin", tokens[1].trim()),
                    _ => {
                        if let Some(path) = found_path {
                            println!("{} is {}", tokens[1], path)
                        } else {
                            println!("{} not found", tokens[1].trim())
                        }
                    }
                }
            }

            _ => println!("{}: command not found", input.trim()),
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
