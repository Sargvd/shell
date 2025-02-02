use crate::tokenizer;
use std::io;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirection: Option<tokenizer::Redirection>,
    pub redirection_target: Option<String>,
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Result<Command, Error> {
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
