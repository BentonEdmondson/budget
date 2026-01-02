use std::{error::Error, fmt};

pub mod add;
pub mod init;

#[derive(Debug)]
struct CommandError {
    message: String,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CommandError {}

impl CommandError {
    fn new(m: &str) -> CommandError {
        CommandError {
            message: m.to_string(),
        }
    }
}