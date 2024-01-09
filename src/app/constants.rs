use const_format::concatcp;

// General
pub const OS: &str = "SE Bastille installer";
pub const VERSION: &str = "0.1.0";

pub const TITRFOQ: &str = concatcp!("Èstalxr d'Èviromàsêrvŷr Bastij", " ", VERSION);
pub const TITLE: &str = concatcp!(OS, " ", VERSION);
