use crate::app::commands::action::CommandAction;

pub struct CommandExecute {}
    
impl CommandExecute {

    pub fn setup_keymap(keymap: &str, keyvar: &str) { 
        CommandAction::setup_keymap(keymap, keyvar)[0].status()
            .unwrap_or_else(|e| panic!("procedure setup keymap failed\n{}", e));
    }

    pub fn clear() {
        CommandAction::clear()[0].status()
            .unwrap_or_else(|e| panic!("procedure clear failed\n{}", e));
    }
}
