#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage

    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_stream = input.trim().split(" ");
        let command = input_stream.next().unwrap();
        match command {
            "exit" => {
                if let Some(arg) = input_stream.next() {
                    if arg == "0" {
                        break;
                    }
                }
                println!("{}: command not found", input.trim())
            }
            "echo" => {
                println!("{}", input_stream.collect::<Vec<&str>>().join(" "))
            }
            _ => {
                println!("{}: command not found", input.trim())
            }
        }
    }

}
