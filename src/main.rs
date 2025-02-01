use std::io::{self, Write};
use std::process::exit;

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input.starts_with("exit") {
            let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
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

        println!("{}: command not found", trimmed_input);
    }
}
