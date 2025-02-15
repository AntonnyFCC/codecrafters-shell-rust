#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let builtin_commands: [String; 3] = [
        String::from("echo"),
        String::from("exit"),
        String::from("type"),
    ];

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => break,
            input if input.starts_with("echo ") => println!("{}", &input[5..]),
            input if input.starts_with("type ") => {
                if builtin_commands.contains(&input[5..].to_string()) {
                    println!("{} is a shell builtin", &input[5..]);
                } else {
                    println!("{} not found", &input[5..]);
                }
            },
            _ =>  println!("{}: command not found", input.trim())
        }

    }
}
