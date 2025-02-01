use std::io::{self, Write};
use std::process::exit;

static BUILTINS: &[&str] = &["exit", "echo", "type"];

fn builtin_exit(input: &str) {
    if input.starts_with("exit") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(code) = parts[1].parse::<i32>() {
                exit(code);
            } else {
                eprintln!("Invalid exit code: {}", parts[1]);
            }
        } else {
            exit(0);
        }
    }
}

fn builtin_echo(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 1 {
        let message = parts[1..].join(" ");
        println!("{}", message);
    } else {
        println!();
    }
}

fn builtin_type(input: &str) {
    let input = &input.split_whitespace().skip(1).next().unwrap_or("");
    if BUILTINS.contains(&input) {
        println!("{} is a shell builtin", input);
    } else {
        println!("{}: not found", input);
    }
}

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        match input.split_whitespace().next() {
            Some(command) if BUILTINS.contains(&command) => match command {
                "exit" => builtin_exit(&input),
                "echo" => builtin_echo(&input),
                "type" => builtin_type(&input),
                _ => println!("{}: command not found", command),
            },
            _ => println!("{}: command not found", input),
        }
    }
}
