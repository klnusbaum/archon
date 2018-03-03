use types::{LogEntry, PeerId, Term};

pub trait StateStore {
    fn get_current_term(&self) -> Term;
    fn set_current_term(&mut self);
    fn get_voted_for(&self) -> PeerId;
    fn set_get_voted_for(&mut self);
    fn log(&self) -> LogIter;
}

pub struct LogIter {}

impl Iterator for LogIter {
    type Item = LogEntry;
    fn next(&mut self) -> Option<LogEntry> {
        None
    }
}
