use strum::EnumString;
use std::{fs, io::{self, Write}, path::PathBuf, str::FromStr};

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CliCommand {
    Exit,
    Echo,
    Type,
    Pwd,
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

