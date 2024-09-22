#[macro_use] extern crate log;

mod app;
mod shared;

use std::{default::Default, env};

// Error handler
use anyhow::Result;

// logger
use log4rs;

// Application
use crate::app::App;

// File locations
const LOC_DIALOGRC: &str = "./dialogrc_gui";

// Environment variables
const ENV_DIALOGRC: &str = "DIALOGRC";

fn main() -> Result<()> {
    log4rs::init_file("./config/log4rs.yml", Default::default()).unwrap();

    // Set color scheme
    env::set_var(ENV_DIALOGRC, LOC_DIALOGRC);
    
    // Start application
    let mut app = App::new();
    app.run()
}
