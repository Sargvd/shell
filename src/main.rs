use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        println!("{}: command not found", input.trim());
    }
}
