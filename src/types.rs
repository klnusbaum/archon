use std::fmt::{Display, Formatter};
use std::fmt;

pub type Command = String;
pub type PeerId = String;
pub type Term = usize;
pub type LogIndex = usize;

#[derive(Debug)]
pub struct LogEntry {
    command: Command,
    term: Term,
}

#[derive(Debug)]
pub struct RaftError {
    message: String,
}

impl RaftError {
    pub fn new(message: String) -> RaftError {
        RaftError { message }
    }
}

impl Display for RaftError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
