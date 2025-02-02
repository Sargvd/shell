mod builtins;
mod exec;
mod parser;
mod tokenizer;
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
                        print!("\x07");
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
