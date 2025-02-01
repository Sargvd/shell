use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, ExitStatus};

static BUILTINS: &[&str] = &["exit", "echo", "type", "pwd"];

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
    let cmd = &input.split_whitespace().skip(1).next().unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    } else {
        if let Ok(path_var) = env::var("PATH") {
            for path in env::split_paths(&path_var) {
                let full_path = Path::new(&path).join(cmd);
                if full_path.exists() {
                    println!("{} is {}", cmd, full_path.display());
                    return;
                }
            }
        }
    }
    println!("{}: not found", cmd);
}

fn builtin_pwd() {
    if let Ok(pwd) = env::current_dir() {
        println!("{}", pwd.display());
    }
}

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
            Some(command) if BUILTINS.contains(&command) => match command {
                "exit" => builtin_exit(&input),
                "echo" => builtin_echo(&input),
                "type" => builtin_type(&input),
                "pwd" => builtin_pwd(),
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
