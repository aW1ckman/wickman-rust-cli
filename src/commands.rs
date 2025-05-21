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

pub fn cmd_echo (input: String) -> String {

    let mut builder: Vec<char> = vec![];
    let mut intermit: Vec<char> = vec![];

    let mut inquotes = false;
    let mut space = false;
    for c in input.chars()  {
        if c.eq(&'\'') {
            if inquotes {
                builder.append(&mut intermit);
                intermit = vec![];
            }
            inquotes = !inquotes;
            space = false;
            continue
        }

        if inquotes {
            intermit.push(c);
        } else {
            if c == ' ' {
                if space {continue;}
                space = true
            } else {space = false}
            builder.push(c);
        }
    }
    
    // Handle case with only one ' ? Assuming treating the ' as a normal char
    if !intermit.is_empty() {
        builder.push('\'');
        builder.append(&mut intermit);
    }

    builder.iter().collect::<String>()
}