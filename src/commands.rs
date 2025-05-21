use strum::EnumString;
use std::{fs, io::{self, Write}, path::PathBuf, str::FromStr};

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CliCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    // Cat,
    Invalid
}


pub struct CliCommandComp<'a> {
    pub command: CliCommand,
    pub orig: &'a str
}

impl <'a> CliCommandComp <'a> {
    fn new(command: CliCommand, orig: &'a str) -> Self {
        Self {
            command,
            orig
        }
    }

    // pub fn builtin_map_command<'b>(stream: &mut std::str::Split<'b, &'static str>) -> CommandComp<'a> where 'b: 'a 

    pub fn builtin_map_command<'b> (command_str: Option<&'b str>) -> CliCommandComp<'a> where 'b: 'a {
        if let Some(command_str) = command_str {
            let command = CliCommand::from_str(command_str).unwrap_or(CliCommand::Invalid);
            CliCommandComp::new(command, command_str)
        } else {
            CliCommandComp::new(CliCommand::Invalid, "")
        }
    }
}

pub fn clean_args (input: String) -> Vec<String> {
    // echo hello 'evening  gamers' hello

    let mut builder: Vec<String> = vec![];
    let mut intermit: String = String::new();

    let mut inquotes = false;
    for c in input.chars()  {
        if c.eq(&'\'') {
            intermit.push(c);
            if inquotes {
                builder.push(intermit);
                intermit = String::new();
            }
            inquotes = !inquotes;
        } else {
            if !inquotes {
                if c == ' ' {
                    if !intermit.is_empty() {
                        builder.push(intermit);
                        intermit = String::new();
                    }
                    // Skip push when space
                    continue;
                }
            }
        intermit.push(c);
        }
    }
    
    // Handle case with only one ' ? Assuming treating the ' as a normal char
    if !intermit.is_empty() {
        builder.push(intermit);
    }

    builder
}