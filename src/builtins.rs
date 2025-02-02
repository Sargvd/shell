use crate::tokenizer;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub static BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

pub fn builtin_exit(args: &[String]) {
    if args.len() == 1 {
        if let Ok(code) = args[0].parse::<i32>() {
            exit(code);
        } else {
            eprintln!("Invalid exit code: {}", args[0]);
        }
    } else {
        exit(0);
    }
}

pub fn builtin_echo(
    args: &[String],
    _redirection: &Option<tokenizer::Redirection>,
    _redirection_target: &Option<String>,
) {
    if args.is_empty() {
        println!();
        return;
    }

    // dbg!(_redirection);
    match _redirection {
        Some(tokenizer::Redirection::File) => {
            let target = _redirection_target.as_ref().unwrap();
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(target)
                .unwrap();
            writeln!(file, "{}", args.join(" ")).unwrap();
        }
        _ => println!("{}", args.join(" ")),
    }
}

pub fn builtin_type(args: &[String]) {
    if args.len() == 0 {
        println!("type requires an argument");
        return;
    }
    let maybe_cmd = &args[0];
    if BUILTINS.contains(&maybe_cmd.as_str()) {
        println!("{} is a shell builtin", maybe_cmd);
        return;
    }
    if let Ok(path_var) = env::var("PATH") {
        for path in env::split_paths(&path_var) {
            let full_path = Path::new(&path).join(&maybe_cmd);
            if full_path.exists() {
                println!("{} is {}", maybe_cmd, full_path.display());
                return;
            }
        }
    }
    println!("{}: not found", maybe_cmd);
}

pub fn builtin_pwd() {
    if let Ok(pwd) = env::current_dir() {
        println!("{}", pwd.display());
    }
}

pub fn builtin_cd(args: &[String]) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    //cd with no parameters gets you $HOME
    if args.len() == 0 {
        if let Ok(home) = env::var("HOME") {
            if let Err(e) = env::set_current_dir(home) {
                eprintln!("{}", e);
                return;
            } else {
                return;
            }
        }
    }
    //Handling ~ as $HOME OR ""
    let mut maybe_path = args[0].clone();
    maybe_path = maybe_path.replace("~", &env::var("HOME").unwrap_or_default());

    let path: &Path = Path::new(maybe_path.as_str());

    if !path.exists() {
        eprintln!("cd: {}: No such file or directory", args[0]);
        return;
    }

    env::set_current_dir(path).expect("Can't change work dir");
}
