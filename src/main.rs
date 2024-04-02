#[macro_use] extern crate log;

mod app;
mod shared;

use std::env;
use std::fs::File;

// Error handler
use anyhow::Result;

// Logger
use simplelog::{CombinedLogger, WriteLogger, LevelFilter, Config};

// Application
use crate::app::App;

// File locations
const LOC_DIALOGRC: &str = "./dialogrc_gui";
const LOC_LOG: &str = "./eqstalxr_bastij.log";

// Environment variables
const ENV_DIALOGRC: &str = "DIALOGRC";

fn main() -> Result<()> {
    // Init logger
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(LOC_LOG).unwrap())
        ]
    ).unwrap();

    // Set color scheme
    env::set_var(ENV_DIALOGRC, LOC_DIALOGRC);
    
    // Start application
    let mut app = App::new();
    app.run()
}
