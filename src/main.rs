use std::io::{self, Write};
use std::process::ExitStatus;
mod builtins;

fn try_exec(input: &str) -> io::Result<ExitStatus> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];
    std::process::Command::new(command).args(args).status()
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
            Some(command) if builtins::BUILTINS.contains(&command) => match command {
                "exit" => builtins::builtin_exit(&input),
                "echo" => builtins::builtin_echo(&input),
                "type" => builtins::builtin_type(&input),
                "pwd" => builtins::builtin_pwd(),
                "cd" => builtins::builtin_cd(&input),
                _ => println!("{}: command not found", command),
            },
            _ => match try_exec(&input) {
                Ok(_) => (),
                Err(_) => {
                    println!("{}: command not found", input);
                }
            },
        }
    }
}
