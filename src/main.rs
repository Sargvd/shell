mod builtins;
mod exec;
mod parser;
mod tokenizer;
use std::fs;
use std::io::{self, Read, Write};
use termios;

fn process_command(cmd: &str) -> io::Result<()> {
    let tokens = tokenizer::tokenize(cmd.to_string())?;
    let cmd = parser::parse(tokens)?;
    exec::exec(cmd)?;
    Ok(())
}

fn read_char() -> io::Result<char> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as u8 as char)
}

fn main() {
    // let stdin = io::stdin();
    let mut termios = termios::Termios::from_fd(0).unwrap();

    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios).unwrap();

    let mut buffer = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        buffer.clear();
        let mut cursor_pos = 0;

        loop {
            match read_char().unwrap() {
                '\t' => {
                    let word = buffer.split_whitespace().last().unwrap_or(&buffer);
                    if let Some(cmd) = builtins::BUILTINS
                        .iter()
                        .find(|&&builtin| builtin.starts_with(word))
                    {
                        let completion = &cmd[word.len()..];
                        buffer.push_str(&completion);
                        buffer.push(' ');
                        cursor_pos += completion.len() + 1;
                        print!("{} ", completion);
                        io::stdout().flush().expect("Failed to flush stdout");
                    } else {
                        let mut matches = Vec::new();

                        // Collect all matching executables from PATH
                        for path in std::env::var("PATH").unwrap_or_default().split(':') {
                            if let Ok(entries) = fs::read_dir(path) {
                                for entry in entries.filter_map(Result::ok) {
                                    if let Some(name) = entry.file_name().to_str() {
                                        if name.starts_with(word) {
                                            matches.push(name.to_string());
                                        }
                                    }
                                }
                            }
                        }

                        // Handle matches
                        if matches.is_empty() {
                            print!("\x07"); // Bell if no matches
                        } else {
                            // Complete with first match
                            let completion = &matches[0][word.len()..];
                            buffer.push_str(completion);
                            buffer.push(' ');
                            cursor_pos += completion.len() + 1;
                            print!("{} ", completion);
                        }
                        io::stdout().flush().expect("Failed to flush stdout");
                    }
                }
                '\n' => {
                    println!();
                    break;
                }
                '\x7f' => {
                    if cursor_pos > 0 {
                        print!("\u{8} \u{8}");
                        io::stdout().flush().expect("Failed to flush stdout");
                        buffer.remove(cursor_pos - 1);
                        cursor_pos -= 1;
                    }
                }
                '\x04' => {
                    builtins::builtin_exit(&["0".to_string()]);
                }
                c => {
                    buffer.insert(cursor_pos, c);
                    cursor_pos += 1;
                    print!("{}", c);
                    io::stdout().flush().expect("Failed to flush stdout");
                }
            }
        }

        if let Err(e) = process_command(&buffer) {
            eprintln!("{}", e);
        }
    }
}
