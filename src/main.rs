mod app;
mod error;
    
use std::env;

use crate::error::Error as Error;
use crate::app::App;

// Debugger
// use env_logger::{Builder, Target};

// File locations
const LOC_DIALOGRC: &str = "./dialogrc_gui";

// Environment variables
const ENV_DIALOGRC: &str = "DIALOGRC";

fn main() -> Result<(), Error> {
    // Init debugger
    // Builder::new().target(Target::Stdout).init();
    
    // Set color scheme
    env::set_var(ENV_DIALOGRC, LOC_DIALOGRC);
    
    // Start application
    let mut app = App::new();
    app.run()
}
