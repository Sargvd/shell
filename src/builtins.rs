use std::env;
use std::path::Path;
use std::process::exit;

pub static BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

pub fn builtin_exit(input: &str) {
    if input.starts_with("exit") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(code) = parts[1].parse::<i32>() {
                exit(code);
            } else {
                eprintln!("Invalid exit code: {}", parts[1]);
            }
        } else {
            exit(0);
        }
    }
}

pub fn builtin_echo(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 1 {
        let message = parts[1..].join(" ");
        println!("{}", message);
    } else {
        println!();
    }
}

pub fn builtin_type(input: &str) {
    let cmd = &input.split_whitespace().skip(1).next().unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    } else {
        if let Ok(path_var) = env::var("PATH") {
            for path in env::split_paths(&path_var) {
                let full_path = Path::new(&path).join(cmd);
                if full_path.exists() {
                    println!("{} is {}", cmd, full_path.display());
                    return;
                }
            }
        }
    }
    println!("{}: not found", cmd);
}

pub fn builtin_pwd() {
    if let Ok(pwd) = env::current_dir() {
        println!("{}", pwd.display());
    }
}

pub fn builtin_cd(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 1 {
        if let Ok(home) = env::var("HOME") {
            if let Err(e) = env::set_current_dir(home) {
                eprintln!("{}", e);
            }
        }
    } else if parts.len() == 2 {
        if !Path::new(parts[1]).exists() {
            eprintln!("cd: {}: No such file or directory", parts[1]);
            return;
        }
        if let Err(e) = env::set_current_dir(parts[1]) {
            eprintln!("{}", e);
        }
    } else {
        eprintln!("cd: too many arguments");
    }
}
