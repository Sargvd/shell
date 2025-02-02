use crate::builtins;
use crate::parser::Command;
use crate::tokenizer;
use std::env;
use std::io;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

pub fn exec(cmd: Command) -> io::Result<ExitStatus> {
    // Handle builtins
    let builtin_result: Option<io::Result<ExitStatus>> = match cmd.name.as_str() {
        "exit" => {
            builtins::builtin_exit(&cmd.args);
            Some(Ok(ExitStatus::from_raw(0)))
        }
        "echo" => {
            builtins::builtin_echo(&cmd.args, &cmd.redirection, &cmd.redirection_target);
            Some(Ok(ExitStatus::from_raw(0)))
        }
        "type" | "pwd" | "cd" => {
            match cmd.name.as_str() {
                "type" => builtins::builtin_type(&cmd.args),
                "pwd" => builtins::builtin_pwd(),
                "cd" => builtins::builtin_cd(&cmd.args),
                _ => unreachable!(),
            }
            Some(Ok(ExitStatusExt::from_raw(0)))
        }
        _ => None,
    };

    if let Some(result) = builtin_result {
        return result;
    }

    // Find command in PATH
    let full_path = env::split_paths(&env::var("PATH").unwrap_or_default())
        .find(|path| path.join(&cmd.name).exists())
        .map(|path| path.join(&cmd.name));

    match full_path {
        Some(path) => path,
        None => {
            println!("{}: command not found", cmd.name);
            return Ok(ExitStatus::from_raw(127));
        }
    };

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
