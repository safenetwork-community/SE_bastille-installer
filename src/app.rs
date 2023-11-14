mod constants;

use const_format::concatcp;
use dialog::DialogBox;
use regex::RegexSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::Error as Error; 
use crate::error::ErrorKind as ErrorKind; 

const TITRFOQ: &str = concatcp!("Èstalxr d sistêmbêstur d Bastij", " ", constants::VERSION);
const TITLE: &str = concatcp!(constants::OS, " ", constants::VERSION);

// General box names
const CANCEL: &str = "cancel";

// General box options
const CLEAR: &str = "clear";

// General box texts
const BOX_TEXT_COLON: &str = ":";

// Input box texts
const BOX_INPUT_TEXT_FULLNAME: &str = "Enter desired Full Name for ";
const BOX_INPUT_TEXT_USERNAME: &str = "Enter the username you want: \n\
    (usernames must be all lowercase and first character may not be a number)"; 
const BOX_INPUT_TEXT_USERGROUPS_1: &str = "Enter additional groups besides the default groups which are\n\
    --> wheel,sys,audio,input,video,storage,lp,network,users,power <--\n\
    for user \'";
const BOX_INPUT_TEXT_USERGROUPS_3: &str = "\' in a comma seperated list:"; 

// Menu box texts
const BOX_MENU_TEXT_DRIVE: &str = "Choose your SDCard/eMMC/USB - Be sure the correct drive is selected!\n\
    WARNING! This WILL destroy the data on it!";
const BOX_MENU_TEXT_KEYMAP_HOST: &str = "Choose your current keyboard layout:";
const BOX_MENU_TEXT_KEYVAR_HOST: &str = "Choose your current keyboard variant:";

// Password box texts
const BOX_PASSWORD_TEXT_USER_SGN: &str = "Enter new Password for ";
const BOX_PASSWORD_TEXT_USER_RPT: &str = "Confirm new Password for ";
const BOX_PASSWORD_TEXT_ROOT_SGN: &str = "Enter new Root Password:";
const BOX_PASSWORD_TEXT_ROOT_RPT: &str = "Confirm new Root Password:";

// Message box texts
const BOX_MSG_FULLNAME_EMPTY: &str = concatcp!("Fullname", constants::BOX_MSG_EMPTY);
const BOX_MSG_USERNAME_EMPTY: &str = concatcp!("Username", constants::BOX_MSG_EMPTY);
const BOX_MSG_USERNAME_INVALID: &str = concatcp!("Username", constants::BOX_MSG_INVALID);
const BOX_MSG_PASSWORD_ROOT_EMPTY: &str = concatcp!("Root password", constants::BOX_MSG_EMPTY);
const BOX_MSG_PASSWORD_ROOT_NOMATCH: &str = concatcp!("Root passwords", constants::BOX_MSG_DONOTMATCH);
const BOX_MSG_PASSWORD_USER_EMPTY: &str = concatcp!("User password", constants::BOX_MSG_EMPTY);
const BOX_MSG_PASSWORD_USER_NOMATCH: &str = concatcp!("User passwords", constants::BOX_MSG_DONOTMATCH);

// Input box names
const FULLNAME: &str = "fullname";
const USERNAME: &str = "username";
const USERGROUPS: &str = "usergroups";

// Menu box names
const DRIVE: &str = "drive";
const KEYMAP_HOST: &str = "keymap_host";
const KEYVAR_HOST: &str = "keyvar_host";

// Password box names 
const PASSWORD_USER_SGN: &str = "password_user_sign";
const PASSWORD_USER_RPT: &str = "password_user_repeat";
const PASSWORD_ROOT_SGN: &str = "password_root_sign";
const PASSWORD_ROOT_RPT: &str = "password_root_repeat";

// Message box names
const FULLNAME_EMPTY: &str = "fullname_empty";
const USERNAME_INVALID: &str = "username_invalid";
const USERNAME_EMPTY: &str = "username_empty";
const PASSWORD_USER_EMPTY: &str = "user_password_empty";
const PASSWORD_USER_NOMATCH: &str = "user_password_nomatch";
const PASSWORD_ROOT_EMPTY: &str = "root_password_empty";
const PASSWORD_ROOT_NOMATCH: &str = "root_password_nomatch";

// Unexpected error messages
const EXP_MBOX: &str = "Could not display message box.";
const EXP_DBOX: &str = "Could not display dialog box.";

// regex
const REGEX_USERNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];

// Exit texts
const MSG_EXIT_CANCEL: &str =   "==> Cancel pressed, exiting..";
const MSG_EXIT_FINISH: &str =   "==> Installation finished! Terminating..";
const MSG_EXIT_CONTACT: &str =  "==> Please contact the owner of this application.";

// Paths
const PATH_BKEYMAP: &str = "/usr/share/bkeymaps";  

// Dimensions
const HEIGHT_BOX_MENU: u32 = 20;
const WIDTH_BOX_MENU: u32 = 50;
const HEIGHT_LIST_MENU: u32 = 15;

pub struct App {
    username: String,
    fullname: String,
    keymap_host: String,
    keyvar_host: String,
    keyvar_path: PathBuf,
    password_user: String,
    password_root: String,
    usergroups: String,
}

impl App {
    
    pub fn new() -> App {
        App {
            username: String::new(),
            fullname: String::new(),
            keymap_host: String::new(),
            keyvar_host: String::new(),
            keyvar_path: PathBuf::new(),
            password_user: String::new(),
            password_root: String::new(),
            usergroups: String::new(),
        }    
    }
     
    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Some(KEYMAP_HOST);
    
        loop {
            match current_box {
                Some(CANCEL) => return Self::cancel(),
                Some(DRIVE) => current_box = Self::handle_dbox_drive(self),
                Some(FULLNAME) => current_box = Self::handle_dbox_fullname(self),
                Some(FULLNAME_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_FULLNAME_EMPTY, FULLNAME),
                Some(KEYMAP_HOST) => current_box = Self::handle_dbox_keymap_host(self),
                Some(KEYVAR_HOST) => current_box = Self::handle_dbox_keyvar_host(self),
                Some(PASSWORD_ROOT_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_EMPTY, PASSWORD_ROOT_SGN),
                Some(PASSWORD_ROOT_NOMATCH) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_NOMATCH, PASSWORD_ROOT_SGN),
                Some(PASSWORD_ROOT_SGN) => current_box = Self::handle_dbox_password_root_sign(self),
                Some(PASSWORD_ROOT_RPT) => current_box = Self::handle_dbox_password_root_repeat(self),
                Some(PASSWORD_USER_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_EMPTY, PASSWORD_USER_SGN),
                Some(PASSWORD_USER_NOMATCH) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_NOMATCH, PASSWORD_USER_SGN),
                Some(PASSWORD_USER_SGN) => current_box = Self::handle_dbox_password_user_sign(self),
                Some(PASSWORD_USER_RPT) => current_box = Self::handle_dbox_password_user_repeat(self),
                Some(USERNAME) => current_box = Self::handle_dbox_username(self),
                Some(USERGROUPS) => current_box = Self::handle_dbox_usergroups(self),
                Some(USERNAME_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_EMPTY, USERNAME),
                Some(USERNAME_INVALID) => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_INVALID, USERNAME),
                Some(_) => return Self::unknown_box(),
                None => return Self::exit(),
            };
        }
    }
 
    fn handle_dbox_username(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 10,
            &BOX_INPUT_TEXT_USERNAME, 
            String::from("bas")) {
            Some(input_text) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_USERNAME).unwrap().is_match(&input_text) {
                        self.username = input_text;
                        return Some(USERGROUPS);
                    }
                    else { return Some(USERNAME_INVALID) }
                }
                else { return Some(USERNAME_EMPTY) }
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_usergroups(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 11, 
            &[BOX_INPUT_TEXT_USERGROUPS_1, &self.username, BOX_INPUT_TEXT_USERGROUPS_3].concat(),
            String::new()) {
            Some(input_text) => {
                self.usergroups = input_text;
                return Some(FULLNAME);
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_fullname(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 11, 
            &[BOX_INPUT_TEXT_FULLNAME, &self.username, BOX_TEXT_COLON].concat(),
            String::from("Useur Bastille")) {
            Some(input_text) => {
                if !input_text.is_empty() {
                    self.fullname = input_text;
                    return Some(PASSWORD_USER_SGN);
                }
                else { return Some(FULLNAME_EMPTY) }
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_password_user_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &[BOX_PASSWORD_TEXT_USER_SGN, &self.username, BOX_TEXT_COLON].concat()) {
            Some(password) => {
                if !password.is_empty() {
                    self.password_user = password;
                    return Some(PASSWORD_USER_RPT);
                }
                else { return Some(PASSWORD_USER_EMPTY); }
            },
            None => return Some(USERNAME),
        };
    }

    fn handle_dbox_password_user_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &[BOX_PASSWORD_TEXT_USER_RPT, &self.username, BOX_TEXT_COLON].concat()) {
            Some(password) => {
                if self.password_user.eq(&password) {
                    return Some(PASSWORD_ROOT_SGN);
                }
                else { return Some(PASSWORD_USER_NOMATCH); }
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_password_root_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &BOX_PASSWORD_TEXT_ROOT_SGN) {
            Some(password) => {
                if !password.is_empty() {
                    self.password_root = password;
                    return Some(PASSWORD_ROOT_RPT);
                }
                else { return Some(PASSWORD_ROOT_EMPTY); }
            },
            None => return Some(CANCEL),
        };
    }
    
    fn handle_dbox_password_root_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &BOX_PASSWORD_TEXT_ROOT_RPT) {
            Some(password) => {
                if self.password_user.eq(&password) {
                    return Some(DRIVE);
                }
                else { return Some(PASSWORD_ROOT_NOMATCH) }
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_drive(&mut self) -> Option<&'static str> {
        match Self::menu_box(&BOX_MENU_TEXT_DRIVE, 
            Self::drivelist_from_lsblk()) {
            Some(_) => {
                return None;
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_keymap_host(&mut self) -> Option<&'static str> {
        match Self::menu_box(&BOX_MENU_TEXT_KEYMAP_HOST, 
            Self::dirlist_from_ls(Path::new(PATH_BKEYMAP))) {
            Some(keymap) => {
                self.keymap_host = keymap.clone();
                self.keyvar_path = Path::new(PATH_BKEYMAP).join(keymap);
                return Some(KEYVAR_HOST);
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_keyvar_host(&mut self) -> Option<&'static str> {
        match Self::menu_box(&BOX_MENU_TEXT_KEYVAR_HOST, 
            Self::filenames_short_from_ls(self.keyvar_path.as_path())) {
            Some(keyvar) => {
                self.keyvar_host = keyvar;
                self.setup_keymap();
                return Some(USERNAME);
            },
            None => return Some(CANCEL),
        };
    }

    fn setup_keymap(&mut self) {
        Command::new("sudo")
        .arg("setup-keymap")
        .arg(self.keymap_host.clone())
        .arg(self.keyvar_host.clone())
        .status()
        .expect("Failed to execute setup-keymap command.");
    }

    // message_box
    fn message_box(width: u32, height: u32, text :&str, return_box :&'static str) -> Option<&'static str> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(width);
        dbox.set_height(height);
        dialog::Message::new(text)
            .show_with(&dbox)
            .expect(EXP_MBOX);
        Some(return_box)
    }
 
    // Input box
    fn input_box(width: u32, height: u32, text: &str, default: String) -> Option<String> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_title(TITLE);
        dbox.set_width(width);
        dbox.set_height(height);

        dialog::Input::new(text) 
        .default(default)
        .show_with(&dbox).expect(EXP_DBOX)
    }

    fn password_box(width: u32, height: u32, text: &str) -> Option<String> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_title(TITLE);
        dbox.set_insecure(true);
        dbox.set_width(width);
        dbox.set_height(height);

        dialog::Password::new(text) 
        .show_with(&dbox).expect(EXP_DBOX)
    }

    fn menu_box(text: &str, list: Vec<[String; 2]>) -> Option<String> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_title(TITLE);
        dbox.set_width(WIDTH_BOX_MENU);
        dbox.set_height(HEIGHT_BOX_MENU);

        dialog::Menu::new(text, HEIGHT_LIST_MENU, list) 
        .show_with(&dbox).expect(EXP_DBOX)
    }
 
    fn cancel() -> Result<(), Error> {
       
        Command::new(CLEAR).status().unwrap();
        println!("{}", MSG_EXIT_CANCEL);
        Ok(())
    }

    fn unknown_box() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }

    fn exit() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        println!("{}", MSG_EXIT_FINISH);
        Ok(())
    }

    fn drivelist_from_lsblk() -> Vec<[String; 2]> {
        let args: [&str; 3] = ["-dn","-o", "NAME"];
        let ls_output = Command::new("lsblk")
        .args(args)
        .output()
        .expect(format!("Failed to execute lsblk {:?}", args).as_str());
 
        let mut drives: Vec<[String; 2]> = Vec::new(); 
        let output_string = String::from_utf8_lossy(&ls_output.stdout);
        for line in output_string.lines() {
           drives.push([String::from(line),String::from("")]); 
        } 
        return drives;
    }

    fn dirlist_from_ls(path: &Path) -> Vec<[String; 2]> {
        let ls_output = Command::new("ls")
        .current_dir(path.to_str().unwrap())
        .output()
        .expect(format!("Failed to execute ls {}", path.to_str().unwrap()).as_str());
 
        let mut dirs: Vec<[String; 2]> = Vec::new(); 
        let output_string = String::from_utf8_lossy(&ls_output.stdout);
        for line in output_string.lines() {
           dirs.push([String::from(line),String::from("")]); 
        } 
        return dirs;
    }

    // Extensionless filenames
    fn filenames_short_from_ls(path: &Path) -> Vec<[String; 2]> {
        let ls_output = Command::new("ls")
        .current_dir(path.to_str().unwrap())
        .output()
        .expect(format!("Failed to execute ls {}", path.to_str().unwrap()).as_str());

        let mut filenames: Vec<[String; 2]> = Vec::new(); 
        let output_string = String::from_utf8_lossy(&ls_output.stdout);
        for line in output_string.lines() {
            filenames.push([Self::shorten_filename(line),
                String::from("")]); 
        } 
        return filenames;
    }
 
    fn shorten_filename(s: &str) -> String {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b'.' {
                return String::from(&s[0..i]);
            }
        }
        String::from(s)
    }
}


