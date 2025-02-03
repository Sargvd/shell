mod builtins;
mod exec;
mod parser;
mod tokenizer;
use std::fs;
use std::io::{self, Read, Write};
use termios;

#[derive(Debug)]
struct Shell {
    buffer: String,
    cursor_pos: usize,
    matches_cache: Vec<String>,
    in_multiple_opts_state: bool,
}

impl Shell {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor_pos: 0,
            matches_cache: Vec::new(),
            in_multiple_opts_state: false,
        }
    }
}

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

fn get_partial_matches(word: &str) -> Vec<String> {
    let mut matches = Vec::new();
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

    for builtin in builtins::BUILTINS
        .iter()
        .filter(|&&builtin| builtin.starts_with(word))
    {
        matches.push(builtin.to_string());
    }
    matches.sort();
    matches.dedup();
    matches
}

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    strs[0]
        .chars()
        .enumerate()
        .take_while(|(i, c)| strs[1..].iter().all(|s| s.chars().nth(*i) == Some(*c)))
        .map(|(_, c)| c)
        .collect()
}

fn process_matches(sh: &mut Shell, matches: &mut Vec<String>) {
    let word = matches.pop().unwrap_or_default();
    // Handle matches
    // dbg!(&matches);
    if matches.is_empty() {
        print!("\x07"); // Bell if no matches
        sh.in_multiple_opts_state = false;
        sh.matches_cache.clear();
    } else if matches.len() == 1 {
        // Complete with first match
        let completion = &matches[0][word.len()..];
        sh.buffer.push_str(completion);
        sh.buffer.push(' ');
        sh.cursor_pos += completion.len() + 1;
        sh.matches_cache.clear();
        sh.in_multiple_opts_state = false;
        print!("{} ", completion);
    } else {
        // Print all matches
        sh.matches_cache = matches.clone();
        sh.matches_cache.sort();
        sh.matches_cache.dedup();
    }
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    let mut termios = termios::Termios::from_fd(0).unwrap();
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios).unwrap();

    let mut sh = Shell::new();

    loop {
        sh.buffer.clear();
        sh.in_multiple_opts_state = false;
        sh.matches_cache.clear();
        sh.cursor_pos = 0;
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        loop {
            match read_char().unwrap() {
                '\t' => {
                    // if sh.in_multiple_opts_state {
                    let word = sh.buffer.split_whitespace().last().unwrap_or(&sh.buffer);
                    let mut matches = get_partial_matches(word);
                    matches.push(word.to_string());
                    process_matches(&mut sh, &mut matches);
                    if !sh.matches_cache.is_empty() {
                        let prefix = longest_common_prefix(&matches);
                        if prefix.len() > sh.buffer.len() {
                            let completion = &prefix[sh.buffer.len()..];
                            sh.buffer.push_str(completion);
                            sh.cursor_pos += completion.len();
                            print!("{}", completion);
                        } else {
                            // dbg!(&sh.in_multiple_opts_state);
                            if !sh.in_multiple_opts_state {
                                print!("\x07");
                                sh.in_multiple_opts_state = true;
                                io::stdout().flush().expect("Failed to flush stdout");
                                continue;
                            }
                            print!("\n");
                            sh.matches_cache.iter().for_each(|m| print!("{}  ", m));
                            println!();
                            print!("$ {}", &sh.buffer);
                            sh.in_multiple_opts_state = false;
                        }
                        io::stdout().flush().expect("Failed to flush stdout");
                        continue;
                    }
                }
                '\n' => {
                    println!();
                    if let Err(e) = process_command(&sh.buffer) {
                        eprintln!("{}", e);
                    }
                    break;
                }
                '\x7f' => {
                    if sh.cursor_pos > 0 {
                        print!("\u{8} \u{8}");
                        io::stdout().flush().expect("Failed to flush stdout");
                        sh.buffer.remove(sh.cursor_pos - 1);
                        sh.cursor_pos -= 1;
                    }
                }
                '\x04' => {
                    builtins::builtin_exit(&["0".to_string()]);
                }
                c => {
                    sh.buffer.insert(sh.cursor_pos, c);
                    sh.cursor_pos += 1;
                    print!("{}", c);
                    io::stdout().flush().expect("Failed to flush stdout");
                }
            }
        }
    }
}
