use std::io::{Error, ErrorKind};

struct TokenizerState {
    in_single_quotes: bool,
    in_double_quotes: bool,
    in_backslash: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Redirection {
    Stdout,
    Stderr,
}

#[derive(Debug)]
pub enum Token {
    Word(String),
    Operand(Redirection),
}

impl TokenizerState {
    fn new() -> Self {
        Self {
            in_single_quotes: false,
            in_double_quotes: false,
            in_backslash: false,
        }
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, Error> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut state = TokenizerState::new();
    let mut out: Vec<Token> = Vec::new();
    let mut current = String::new();

    for c in input.chars() {
        match (
            c,
            state.in_backslash,
            state.in_single_quotes,
            state.in_double_quotes,
        ) {
            // Single quote handling
            // Outside double quotes, no backslash, toggle single quotes
            ('\'', false, _, false) => state.in_single_quotes = !state.in_single_quotes,
            // Backlash captures single quote as a literal
            ('\'', true, _, false) => {
                current.push(c);
                state.in_backslash = false;
            }
            // Inside double quotes, treat single quote as literal
            ('\'', false, _, true) => {
                current.push(c);
            }

            // Double quote handling
            // Outside single quotes, no backslash, toggle double quotes
            ('"', false, false, _) => state.in_double_quotes = !state.in_double_quotes,
            // Backlash captures double quote as a literal
            ('"', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            // Inside single quotes, treat double quote as literal
            ('"', false, true, _) => {
                current.push(c);
            }

            // Space handling
            // If not backslash, not in quotes, and not empty, push current token
            (' ', false, false, false) if !current.is_empty() => {
                if !current.is_empty() {
                    if current == ">" || current == "1>" {
                        out.push(Token::Operand(Redirection::Stdout));
                    } else if current == "2>" {
                        out.push(Token::Operand(Redirection::Stderr));
                    } else {
                        out.push(Token::Word(current.clone()));
                    }
                    current = String::new();
                }
            }
            // If not backslash, not in quotes, and empty, skip
            (' ', false, false, false) => continue,
            // If backslash, push space as a literal & turn off backslash
            (' ', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            // If in single or double quotes, treat space as a literal
            (' ', false, true, _) | (' ', false, _, true) => current.push(c),

            // Backslash handling
            // If backslash in single quotes, treat as a literal
            ('\\', false, true, _) => {
                current.push(c);
            }
            // If backslash in double quotes and not in backlash, turn on backslash
            ('\\', false, _, true) => {
                state.in_backslash = true;
            }
            // If second backslash, treat as a literal & turn off
            ('\\', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }

            // If not backslash, turn on backslash
            ('\\', false, _, _) => state.in_backslash = true,

            // Regular characters
            (c, true, _, true) => {
                current.push('\\');
                current.push(c);
                state.in_backslash = false;
            }
            (c, true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            (c, false, _, _) => current.push(c),
        }
    }

    if !current.is_empty() {
        if current == ">" || current == "1>" {
            out.push(Token::Operand(Redirection::Stdout));
        } else if current == "2>" {
            out.push(Token::Operand(Redirection::Stderr));
        } else {
            out.push(Token::Word(current));
        }
    }

    if state.in_single_quotes {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Unmatched single quote",
        ));
    }
    if state.in_double_quotes {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Unmatched double quote",
        ));
    }

    // dbg!(&out);
    Ok(out)
}
