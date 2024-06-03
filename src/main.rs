use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
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
            "exit" => {
                // TODO: handle the case where tokens[1] is undefined
                let code = tokens[1].parse::<i32>();

                if let Ok(num) = code {
                    exit(num)
                } else {
                    println!("enter valid integer")
                }
            }

            "echo" => println!("{}", tokens[1..].join(" ")),

            "type" => match tokens[1] {
                "echo" => println!("{} is a shell builtin", tokens[1].trim()),
                "exit" => println!("{} is a shell builtin", tokens[1].trim()),
                "type" => println!("{} is a shell builtin", tokens[1].trim()),
                _ => {
                    if let Some(path) = find_exe(tokens[1]) {
                        println!("{} is {}", tokens[1], path.to_str().unwrap())
                    } else {
                        println!("{} not found", tokens[1].trim())
                    }
                }
            },

            _ => {
                if let Some(path) = find_exe(tokens[0]) {
                    Command::new(path)
                        .args(&tokens[1..])
                        .status()
                        .expect("failed to execute process");
                } else {
                    println!("{}: command not found", input.trim())
                }
            }
        }

        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
