pub const MSG_EXIT_CONTACT: &str =  "\nPlease contact the owner of this application.";

#[derive(Debug)]
pub enum ErrorInstaller {
    NotFoundBox(),
    FailedCommand(String),
    EmptyMenu(),
    Unknown(String),
}

impl std::error::Error for ErrorInstaller {}

impl std::fmt::Display for ErrorInstaller {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotFoundBox() => 
                write!(f, "Unknown box selected{MSG_EXIT_CONTACT}"),
            Self::FailedCommand(s) => 
                write!(f, "External command fails\n\n{s}{MSG_EXIT_CONTACT}"), 
            Self::EmptyMenu() => 
                write!(f, "Menu box is empty{MSG_EXIT_CONTACT}"),
            Self::Unknown(s) => 
                write!(f, "An unknown error has occured\n\n{s}{MSG_EXIT_CONTACT}"),
       }
    }
}
