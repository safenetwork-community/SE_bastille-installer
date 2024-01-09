use std::path::{Path, PathBuf};
use std::process::Command;

// General commands
pub const CLEAR: &str = "clear";
pub const SUDO: &str = "sudo";
// pub const UMOUNT: &str = "umount";
pub const SETUP_KEYMAP: &str = "setup-keymap";
// pub const MOUNT: &str = "mount";

pub const TMP_DIR: &str = "/var/tmp/installateur-bastille";

pub struct CommandInstall {}
    
impl CommandInstall {
    
    pub fn prepare_card(drive: &str) {
        Self::umount(Path::new(TMP_DIR)
            .join(Path::new(drive)))
    }

    pub fn create_install() {
        Command::new("sleep")
        .arg("0.2")
        .status()
        .expect(Self::expect("sleep").as_str());
    }

    pub fn cleanup() {
        /*
        Command::new(SUDO)
        .arg(UMOUNT)
        .arg("/boot")
        .status()
        .expect(Self::expect(UMOUNT).as_str());
        */
    }

    pub fn umount(dir: PathBuf) {
        println!("{:?}", dir);
        /*
        Command::new(SUDO)
        .arg(UMOUNT)
        .arg(dir.as_os_str()) 
        .status()
        .expect(Self::expect(UMOUNT).as_str());
        */
    }

    pub fn clear() {
        Command::new(CLEAR)
        .status()
        .expect(Self::expect(CLEAR).as_str());
    }

    pub fn setup_keymap(keymap: &str, keyvar: &str) {
        Command::new(SUDO)
        .arg(SETUP_KEYMAP)
        .arg(keymap)
        .arg(keyvar)
        .status()
        .expect(Self::expect(SETUP_KEYMAP).as_str());
    }

    pub fn show_elapsed_time() {
        Command::new("sleep")
        .arg("0.1")
        .status()
        .expect(Self::expect("sleep").as_str());
    }

    pub fn expect(comm: &str) -> String {
        format!("Failed to execute {} command", comm)
    }
}
