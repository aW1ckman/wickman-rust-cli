use strum::EnumString;
use std::{fs, io::{self, Write}, path::PathBuf, str::FromStr};

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Command {
    Exit,
    Echo,
    Type,
    Invalid
}


pub struct CommandComp<'a> {
    pub command: Command,
    pub orig: &'a str
}

impl <'a> CommandComp <'a> {
    fn new(command: Command, orig: &'a str) -> Self {
        Self {
            command,
            orig
        }
    }

    // pub fn builtin_map_command<'b>(stream: &mut std::str::Split<'b, &'static str>) -> CommandComp<'a> where 'b: 'a 

    pub fn builtin_map_command<'b> (command_str: Option<&'b str>) -> CommandComp<'a> where 'b: 'a {
        if let Some(command_str) = command_str {
            let command = Command::from_str(command_str).unwrap_or(Command::Invalid);
            CommandComp::new(command, command_str)
        } else {
            CommandComp::new(Command::Invalid, "")
        }
    }
}

