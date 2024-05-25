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

        let input = input.trim();
        let input_list = input.split(" ").collect::<Vec<&str>>();
        let commands = vec!["echo", "exit", "type"];

        match input_list[0] {
            "exit" => exit(0),
            "echo" => {
                let echo_value = &input_list[1..].join(" ");
                echo(&echo_value);
            }
            "type" => {
                let type_value = &input_list[1];
                builtin_check(type_value, &commands);
            }
            _ => print!("{}: command not found\n", input),
        }
    }
}

fn echo(value: &str) {
    println!("{}", value);
}

fn builtin_check(value: &str, cmd_list: &Vec<&str>) {
    let does_have;
    if cmd_list.contains(&value) {
        does_have = true;
    } else {
        does_have = false;
    }
    match does_have {
        true => println!("{value} is a shell builtin"),
        false => println!("{value} not found"),
    }
}
