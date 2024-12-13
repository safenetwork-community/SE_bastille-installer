use duct::cmd;

pub struct CommandExecute {}
    
impl CommandExecute {

    pub fn setup_keymap(keymap: &str, keyvar: &str) {
        match cmd!("sudo", "loadkeys", keymap, keyvar).run() {
            Ok(_) => {},
            Err(e) => panic!("procedure setup keymap failed\n{}", e),
        }
    }

    pub fn clear() {
        match cmd!("clear").run() {
            Ok(_) => {},
            Err(e) => panic!("procedure clear failed\n{}", e),
        }
    }
}
