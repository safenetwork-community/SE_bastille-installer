use std::process::Command as Comm;

pub struct Command {}
    
impl Command {
    pub fn setup_keymap(keymap: &str, keyvar: &str) {
        Comm::new("sudo")
        .arg("setup-keymap")
        .arg(keymap)
        .arg(keyvar)
        .status()
        .expect("Failed to execute setup-keymap command.");
    }
}
