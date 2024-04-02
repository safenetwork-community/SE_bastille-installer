use crate::app::commands::action::CommandAction;

pub struct CommandExecute {}
    
impl CommandExecute {

    pub fn setup_keymap(keymap: &str, keyvar: &str) { 
        CommandAction::setup_keymap(keymap, keyvar).unwrap().status().expect("Procedure clear failed");
    }

    pub fn clear() {
        CommandAction::clear().unwrap().status().expect("Procedure clear failed");
    }
}
