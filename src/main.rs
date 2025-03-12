#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;
use pathsearch::find_executable_in_path;
use regex::Regex;

const BUILTINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => break,
            "pwd" => pwd_command(),
            input if input.starts_with("cd ")=> cd_command(&input[3..]),
            input if input.starts_with("echo ") => echo_command(&input[5..]),
            input if input.starts_with("type ") => type_command(&input[5..]),
            input => {
                let program_and_arguments: Vec<&str> = input.split_whitespace().collect();

                if let Some(executable) = find_executable_in_path(program_and_arguments[0]) {
                    executable_commnad(executable, &program_and_arguments[1..]);
                } else {
                    print_not_found(program_and_arguments[0]);
                }
            }
        }
    }
}

fn print_not_found(command: &str) {
    println!("{}: not found", command);
}

fn print_builtin(command: &str) {
    println!("{} is a shell builtin", command);
}

fn type_command(argument: &str) {
    if BUILTINS.contains(&argument) {
        print_builtin(argument);
    } else if let Some(executable) = find_executable_in_path(argument) {
        println!("{} is {}", argument, executable.display())
    } else {
        print_not_found(argument);
    }
}

fn echo_command(argument: &str) {
    let re_spaces = Regex::new(r"('[^']*')|\s+").unwrap();
    let re_quotes = Regex::new(r"'([^']*)'").unwrap();

    let without_spaces = re_spaces.replace_all(argument, |caps: &regex::Captures| {
        if caps.get(1).is_some() {
            caps[1].to_string()
        } else {
            " ".to_string()
        }
    });
    let result = re_quotes.replace_all(&without_spaces, "$1");
    result.to_string();
    println!("{}", result);
}

fn executable_commnad(executable: PathBuf, arguments: &[&str]) {
    if let Some(name) = executable.file_name() {
        let output = Command::new(name).args(arguments).output().unwrap();
        if output.status.success() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

fn pwd_command() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error getting the current directory: {}", e)
    }
}

fn cd_command(argument: &str) {
    let path = PathBuf::from(argument);
    if argument == "~" {
        let home_dir = env::var("HOME").expect("Error getting home var");
        let _ = env::set_current_dir(Path::new(&home_dir));
    } else if path.exists() {
        let _ = env::set_current_dir(&path);
    } else {
        println!("cd: {}: No such file or directory", argument);
    }
}
