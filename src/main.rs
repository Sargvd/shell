mod builtins;
mod exec;
mod parser;
mod tokenizer;
use std::io;
use std::io::Write;

fn process_command(cmd: &str) -> io::Result<()> {
    let tokens = tokenizer::tokenize(cmd.to_string())?;
    let cmd = parser::parse(tokens)?;
    exec::exec(cmd)?;
    Ok(())
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        if let Err(e) = process_command(input.trim()) {
            eprintln!("{}", e);
        }
    }
}
