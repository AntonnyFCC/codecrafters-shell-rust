#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use pathsearch::find_executable_in_path;

const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

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
    println!("{}", argument);
}

fn executable_commnad(executable: PathBuf, arguments: &[&str]) {
    if let Some(name) = executable.file_name() {
        let output = Command::new(name).args(arguments).output().unwrap();
        if output.status.success() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

