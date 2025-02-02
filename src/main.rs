use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitStatus;
mod builtins;
mod tokenizer;
use std::io::Error;

#[derive(Debug, Clone)]
struct Command {
    name: String,
    args: Vec<String>,
    redirection: Option<tokenizer::Redirection>,
    redirection_target: Option<String>,
}

fn parse(tokens: Vec<tokenizer::Token>) -> Result<Command, Error> {
    // dbg!(&tokens);
    let mut set_redirection = false;
    let mut cmd = Command {
        name: String::new(),
        args: Vec::new(),
        redirection: None,
        redirection_target: None,
    };

    for token in tokens {
        match (token, set_redirection) {
            (tokenizer::Token::Word(word), false) => {
                if cmd.name.is_empty() {
                    cmd.name = word;
                } else {
                    cmd.args.push(word);
                }
            }
            (tokenizer::Token::Word(word), true) => {
                cmd.redirection_target = Some(word);
                set_redirection = false;
            }
            (tokenizer::Token::Operand(op), false) => {
                if cmd.redirection.is_none() {
                    cmd.redirection = Some(op);
                    if op == tokenizer::Redirection::File {
                        set_redirection = true;
                    } else if op == tokenizer::Redirection::Stdout {
                        cmd.redirection_target = None;
                    }
                } else {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        "Multiple redirections not supported",
                    ));
                }
            }
            (tokenizer::Token::Operand(_), true) => {
                return Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    "Redirection target expected",
                ));
            }
        }
    }

    Ok(cmd)
}

fn exec(cmd: Command) -> io::Result<ExitStatus> {
    match cmd.name.as_str() {
        "exit" => builtins::builtin_exit(&cmd.args),
        "echo" => {
            builtins::builtin_echo(&cmd.args, &cmd.redirection, &cmd.redirection_target);
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
        }
        "type" => {
            builtins::builtin_type(&cmd.args);
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
        }
        "pwd" => {
            builtins::builtin_pwd();
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
        }
        "cd" => {
            builtins::builtin_cd(&cmd.args);
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
        }
        _ => (),
    }

    let path_var = env::var("PATH").unwrap_or_default();
    let mut full_path_cmd = None;
    for path in env::split_paths(&path_var) {
        let full_path = Path::new(&path).join(&cmd.name);
        if full_path.exists() {
            full_path_cmd = Some(full_path);
        }
    }

    if full_path_cmd.is_none() {
        println!("{}: command not found", cmd.name);
        return Ok(std::os::unix::process::ExitStatusExt::from_raw(127));
    }

    // let mut command = std::process::Command::new(&full_path_cmd.unwrap());
    let mut command = std::process::Command::new(&cmd.name);
    command.args(&cmd.args);
    if let Some(redirection) = cmd.redirection {
        match redirection {
            tokenizer::Redirection::Stdout => {
                command.stdout(std::process::Stdio::piped());
            }
            tokenizer::Redirection::File => {
                if let Some(target) = cmd.redirection_target.as_ref() {
                    command.stdout(std::fs::File::create(target)?);
                } else {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        "Redirection target expected",
                    ));
                }
            }
        }
    }

    // dbg!(&command);
    command.status()
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let tokens = tokenizer::tokenize(input.trim().to_string());
        match tokens {
            Ok(tokens) => {
                let cmd = parse(tokens);
                // dbg!(&cmd);
                match cmd {
                    Ok(cmd) => {
                        exec(cmd).expect("Failed to execute command");
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
