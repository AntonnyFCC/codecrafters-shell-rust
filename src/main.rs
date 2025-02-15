#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let mut words = trimmed.splitn(2, " ");
        let command_word = words.next().unwrap_or("");
        let param_word = words.next().unwrap_or("");

        match trimmed {
            "exit 0" => break,
            _ => {
                match command_word {
                    "echo" => {
                        println!("{}", param_word);
                    },
                    _ => {
                        println!("{}: command not found", input.trim());
                    }
                }
            }
        }

    }
}
