use std::io::{self, Write};
use std::process::ExitStatus;
mod builtins;
use std::io::Error;

fn try_exec(input: &Vec<String>) -> io::Result<ExitStatus> {
    let maybe_command = &input[0];
    let args = &input[1..];
    for path in std::env::var("PATH").unwrap().split(':') {
        let full_path = std::path::Path::new(path).join(maybe_command);
        if full_path.exists() {
            return std::process::Command::new(maybe_command)
                .args(args)
                .status();
        }
    }
    Err(Error::new(
        io::ErrorKind::NotFound,
        format!("{}: command not found", maybe_command),
    ))
}

fn tokenize(input: String) -> Result<Vec<String>, Error> {
    let mut in_s_quotes: bool = false;
    let mut out: Vec<String> = vec![];
    let mut current = String::new();
    for c in input.chars() {
        match c {
            '\'' => in_s_quotes = !in_s_quotes,
            ' ' if !in_s_quotes => {
                if current.is_empty() {
                    continue;
                }
                out.push(current.clone());
                current = String::new();
            }
            ' ' if in_s_quotes => {
                current.push(c);
            }
            _ => current.push(c),
        }
    }
    out.push(current.clone());
    if in_s_quotes {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "Unmatched single quote",
        ));
    }
    // dbg!(&out);
    Ok(out.clone())
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let tokens = tokenize(input.trim().to_string());
        match tokens {
            Ok(tokens) => match tokens[0].as_str() {
                "exit" => builtins::builtin_exit(&tokens[1..]),
                "echo" => builtins::builtin_echo(&tokens[1..]),
                "type" => builtins::builtin_type(&tokens),
                "pwd" => builtins::builtin_pwd(),
                "cd" => builtins::builtin_cd(&tokens[1..]),
                _ => match try_exec(&tokens) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
