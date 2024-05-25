#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();

        let _ = io::stdout().flush();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            exit(0);
        } else {
            print!("{}: command not found\n", input.trim());
        }
    }
}
