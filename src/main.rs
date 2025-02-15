#[allow(unused_imports)]
use std::io::{self, Write};
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
            input => print_not_found(&input),
        }
    }
}

fn print_not_found(command: &str) {
    println!("{}: not found", command);
}

fn print_builtin(command: &str) {
    println!("{} is shell builtin", command);
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
