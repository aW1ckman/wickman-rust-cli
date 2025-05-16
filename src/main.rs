#[allow(unused_imports)]

mod commands;

use commands::Command;
use std::{io::{self, Write}, str::FromStr};

struct CommandComp<'a> {
    command: Command,
    orig: &'a str
}

impl <'a> CommandComp <'a> {
    fn new(command: Command, orig: &'a str) -> Self {
        Self {
            command,
            orig
        }
    }
}

fn spit_command<'a>(stream: &mut std::str::Split<'a, &'static str>) -> CommandComp<'a> {
    let command_str = stream.next();
    if let Some(command_str) = command_str {
        let command = Command::from_str(command_str).unwrap_or(Command::Invalid);
        CommandComp::new(command, command_str)
    } else {
        CommandComp::new(Command::Invalid, "")
    }
}

fn main() {
    // Uncomment this block to pass the first stage

    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_stream: std::str::Split<'_, &'static str> = input.trim().split(" ");
        let command = spit_command(&mut input_stream).command;
        match command {
            Command::Type => {
                let c: CommandComp<'_> = spit_command(&mut input_stream);
                match c.command {
                    Command::Invalid => {
                        println!("{}: not found", c.orig)
                    }
                    _ => {
                        println!("{} is a shell builtin", c.orig)
                    }
                }
            }
            Command::Echo => {
                println!("{}", input_stream.collect::<Vec<&str>>().join(" "))
            }
            Command::Exit => {
                if let Some(arg) = input_stream.next() {
                    if arg == "0" {
                        break;
                    }
                }
                println!("{}: command not found", input.trim())
            }
            Command::Invalid => {
                println!("{}: command not found", input.trim())
            }
        }
    }

}
