#[allow(unused_imports)]

mod commands;

use commands::{cmd_echo, CliCommand, CliCommandComp};
use std::{collections::HashMap, fs, io::{self, Write}, path::PathBuf};

fn load_path_cmds() -> HashMap<String, PathBuf> {
    let paths_str = std::env::var_os("PATH").unwrap();
    let mut paths: HashMap<String, PathBuf> = HashMap::new();
    for path in std::env::split_paths(&paths_str) {
        if path.is_dir() {
            let executables = match fs::read_dir(path) {
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
                                if !is_linux_binary {filename = filename.rsplit_once(".").unwrap().0.to_owned();}
                                if let None = paths.get(&filename) {
                                    paths.insert(filename, exec.path());
                                }
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
    // Preload env vars
    let paths = load_path_cmds();
    let home_binding = std::env::var_os("HOME").unwrap_or_default();
    let home: &str = home_binding.to_str().unwrap_or("/");

    loop {

        // Wait for user input
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_stream: std::str::Split<'_, &'static str> = input.trim().split(" ");
        let command = CliCommandComp::builtin_map_command(input_stream.next());

        // Find command
        match command.command {
            CliCommand::Type => {
                let c: CliCommandComp<'_> = CliCommandComp::builtin_map_command(input_stream.next());
                match c.command {
                    CliCommand::Invalid => {
                        // Check if executable in PATH
                        let cmd = c.orig.to_lowercase();
                        if let Some(path) = paths.get(&cmd) {
                            println!("{cmd} is {path}", path=path.to_str().unwrap());
                            continue;
                        }
                        // Not builtin or in path
                        eprintln!("{}: not found", c.orig);
                    }
                    _ => {
                        println!("{} is a shell builtin", c.orig)
                    }
                }
            }
            CliCommand::Echo => {
                // tokenise by quotation marks
                let input = input_stream.collect::<Vec<&str>>().join(" ");
                let res = cmd_echo(input);
                println!("{}", res);
            }
            // CliCommand::Cat => {

            // }
            CliCommand::Exit => {
                if let Some(arg) = input_stream.next() {
                    if arg == "0" {
                        break;
                    }
                }
                eprintln!("{}: command not found", command.orig)
            }
            CliCommand::Pwd => println!("{}", std::env::current_dir().unwrap_or_default().display()),
            CliCommand::Cd => {
                let mut path = input_stream.next().unwrap_or("/");
                if path == "~" { path = home }
                let res = std::env::set_current_dir(path);
                if let Ok(_) = res {continue;};
                eprintln!("cd: {}: No such file or directory", path);
            }
            CliCommand::Invalid => {
                // Check if executable in PATH
                let cmd = command.orig.to_lowercase();
                if paths.contains_key(&cmd) {
                    use std::process::Command;
                    let process = Command::new(command.orig)
                        .args(input_stream)
                        .spawn();
                    match process {
                        Ok(mut process) => {
                            let end = process.wait();
                            if let Err(e) = end {
                                eprintln!("error running executable: {}\nError: {e:#?}", command.orig)
                            }
                        },
                        Err(e) => eprintln!("error running executable: {}\nError: {e:#?}", command.orig),
                    }
                    
                } else {
                    eprintln!("{}: command not found", command.orig)
                }
            }
        }
    }
}
