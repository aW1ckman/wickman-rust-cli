#[allow(unused_imports)]

mod commands;

use commands::{Command, CommandComp};
use std::{collections::HashMap, fs, io::{self, Write}, path::PathBuf};

fn load_path_cmds() -> HashMap<String, String> {
    let mut file_seperator = "/";
    if cfg!(windows) {
        file_seperator = "\\";
    }
    let paths_str = std::env::var_os("PATH").unwrap();
    let mut paths: HashMap<String, String> = HashMap::new();
    for path in std::env::split_paths(&paths_str) {
        if path.is_dir() {
            let executables = match fs::read_dir(path.clone()) {
                Ok(ent) => ent,
                Err(_) => continue,
            };
            for exec in executables {
                match exec {
                    Ok(exec) => {
                        if !exec.path().is_dir() {
                            let mut filename = exec.file_name().to_str().unwrap().to_owned().to_lowercase();
                            let is_linux_binary = !filename.contains(".");
                            if filename.ends_with(".exe") || filename.ends_with(".bin") || is_linux_binary {
                                let p: String = path.to_str().unwrap().to_owned() + file_seperator + &filename;
                                if !is_linux_binary {filename = filename.rsplit_once(".").unwrap().0.to_owned();}
                                paths.insert(filename, p);
                            }
                        }
                    }
                    Err(_) => continue,
                };
            }
        }
    }
    paths
}

fn main() {
    let paths = load_path_cmds();

    loop {

        // Wait for user input
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_stream: std::str::Split<'_, &'static str> = input.trim().split(" ");
        let command = CommandComp::builtin_map_command(input_stream.next()).command;

        // Find command
        match command {
            Command::Type => {
                let c: CommandComp<'_> = CommandComp::builtin_map_command(input_stream.next());
                match c.command {
                    Command::Invalid => {
                        let cmd = c.orig.to_lowercase();
                        if let Some(path) = paths.get(&cmd) {
                            println!("{cmd} is {path}");
                            continue;
                        }
                        // Not builtin or in path
                        println!("{}: not found", c.orig);
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
