use std::io::{Error, ErrorKind};

struct TokenizerState {
    in_s_quotes: bool,
    in_d_quotes: bool,
    in_backslash: bool,
}

impl TokenizerState {
    fn new() -> Self {
        Self {
            in_s_quotes: false,
            in_d_quotes: false,
            in_backslash: false,
        }
    }
}

pub fn tokenize(input: String) -> Result<Vec<String>, Error> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut state = TokenizerState::new();
    let mut out = Vec::with_capacity(input.split_whitespace().count());
    let mut current = String::new();

    for c in input.chars() {
        match (c, state.in_backslash, state.in_s_quotes, state.in_d_quotes) {
            // Single quote handling
            ('\'', false, _, false) => state.in_s_quotes = !state.in_s_quotes,
            ('\'', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            ('\'', false, _, true) => current.push(c),

            // Double quote handling
            ('"', false, false, _) => state.in_d_quotes = !state.in_d_quotes,
            ('"', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }

            // Space handling
            (' ', false, false, false) if !current.is_empty() => {
                out.push(std::mem::take(&mut current));
            }
            (' ', false, false, false) => continue,
            (' ', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            (' ', false, true, _) | (' ', false, _, true) => current.push(c),

            // Backslash handling
            ('\\', true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            ('\\', false, _, _) => state.in_backslash = true,

            // Special escaped characters
            ('n', true, _, _) => {
                current.push('\n');
                state.in_backslash = false;
            }
            ('t', true, _, _) => {
                current.push('\t');
                state.in_backslash = false;
            }

            // Regular characters
            (c, true, _, _) => {
                current.push(c);
                state.in_backslash = false;
            }
            (c, false, _, _) => current.push(c),
        }
    }

    if !current.is_empty() {
        out.push(current);
    }

    if state.in_s_quotes {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Unmatched single quote",
        ));
    }
    if state.in_d_quotes {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Unmatched double quote",
        ));
    }

    Ok(out)
}
