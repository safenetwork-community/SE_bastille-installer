use const_format::concatcp;

// General
pub const OS: &str = "SE Bastille installer";
pub const VERSION: &str = "0.1.0";

// Message box texts
pub const BOX_MSG_PLEASE: &str = "\n\
    \n\
    \t\t\tPlease try again";
pub const BOX_MSG_EMPTY: &str = concatcp!(" cannot be empty", BOX_MSG_PLEASE);
pub const BOX_MSG_INVALID: &str = concatcp!(" contains invalid characters", BOX_MSG_PLEASE);
pub const BOX_MSG_DONOTMATCH: &str = concatcp!(" do not match!", BOX_MSG_PLEASE);
