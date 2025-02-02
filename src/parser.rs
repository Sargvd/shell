use crate::tokenizer;
use std::io;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirection: Option<tokenizer::Redirection>,
    pub redirection_target: Option<String>,
    pub stderr_redirection: Option<tokenizer::Redirection>,
    pub stderr_redirection_target: Option<String>,
    pub stdout_append: Option<tokenizer::Redirection>,
    pub stderr_append: Option<tokenizer::Redirection>,
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Result<Command, Error> {
    // dbg!(&tokens);
    let mut set_redirection = false;
    let mut cmd = Command {
        name: String::new(),
        args: Vec::new(),
        redirection: None,
        redirection_target: None,
        stderr_redirection: None,
        stderr_redirection_target: None,
        stdout_append: None,
        stderr_append: None,
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
                if !cmd.redirection.is_none() {
                    cmd.redirection_target = Some(word);
                } else if !cmd.stderr_redirection.is_none() {
                    cmd.stderr_redirection_target = Some(word);
                } else if !cmd.stdout_append.is_none() {
                    cmd.redirection_target = Some(word);
                } else if !cmd.stderr_append.is_none() {
                    cmd.stderr_redirection_target = Some(word);
                } else {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        "Redirection target expected",
                    ));
                }
                set_redirection = false;
            }
            (tokenizer::Token::Operand(op), false) => match op {
                tokenizer::Redirection::Stderr => {
                    if cmd.stderr_redirection.is_none() {
                        cmd.stderr_redirection = Some(op);
                        set_redirection = true;
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Multiple stderr redirections not supported",
                        ));
                    }
                }
                tokenizer::Redirection::Stdout => {
                    if cmd.redirection.is_none() {
                        cmd.redirection = Some(op);
                        set_redirection = true;
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Multiple redirections not supported",
                        ));
                    }
                }
                tokenizer::Redirection::StdoutAppend => {
                    if !cmd.redirection.is_none() {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Mixing append and stdout redirections not supported",
                        ));
                    } else if cmd.stdout_append.is_none() {
                        cmd.stdout_append = Some(op);
                        set_redirection = true;
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Multiple redirections not supported",
                        ));
                    }
                }
                tokenizer::Redirection::StderrAppend => {
                    if !cmd.stderr_redirection.is_none() {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Mixing append and stderr redirections not supported",
                        ));
                    } else if cmd.stderr_append.is_none() {
                        cmd.stderr_append = Some(op);
                        set_redirection = true;
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            "Multiple redirections not supported",
                        ));
                    }
                }
            },
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
