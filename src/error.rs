use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    BoxNotFound(),
    EmptyMenu(),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::BoxNotFound() => "Occurs when dialogbox can not be found.",
            ErrorKind::EmptyMenu() => "Occurs when dialogbox has an empty menu list.",
        }
    }
}
 
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::BoxNotFound() => 
                write!(f, "Unknown box selected"),
            ErrorKind::EmptyMenu() => 
                write!(f, "Menu box is empty"),

       }
    }
}
