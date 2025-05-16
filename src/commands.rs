use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Command {
    Exit,
    Echo,
    Type,
    Invalid
}