mod app;
mod error;
    
use std::env;

use crate::error::Error as Error;
use crate::app::App;

// File locations
const LOC_DIALOGRC: &str = "./dialogrc_gui";

// Environment variables
const ENV_DIALOGRC: &str = "DIALOGRC";

fn main() -> Result<(), Error> {
    env::set_var(ENV_DIALOGRC, LOC_DIALOGRC);
    let mut app = App::new();
    app.run()
}
