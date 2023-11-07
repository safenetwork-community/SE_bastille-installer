use const_format::concatcp;
use dialog::DialogBox;
use regex::RegexSet;
use std::process::Command;

use crate::error::Error as Error; 
use crate::error::ErrorKind as ErrorKind; 

const VERSION: &str = "0.1.0";
const TITRFOQ: &str = concatcp!("Èstalxr d sistêmbêstur d Bastij", " ", VERSION);
const OS: &str = "SE Bastille installer";
const TITLE: &str = concatcp!(OS, " ", VERSION);

// General box texts
const BOX_TEXT_COLON: &str = ":";

// Input box texts
const BOX_INPUT_TEXT_FULLNAME: &str = "Enter desired Full Name for";
const BOX_INPUT_TEXT_USERNAME: &str = "Enter the username you want: \n\
    (usernames must be all lowercase and first character may not be a number)"; 
const BOX_INPUT_TEXT_USERGROUPS_1: &str = "Enter additional groups besides the default groups which are\n\
    --> wheel,sys,audio,input,video,storage,lp,network,users,power <--\n\
    for user \'";
const BOX_INPUT_TEXT_USERGROUPS_3: &str = "\' in a comma seperated list:"; 

// Pawword box texts
const BOX_PASSWORD_TEXT_USER_SGN: &str = "Enter new Password for ";
const BOX_PASSWORD_TEXT_USER_RPT: &str = "Confirm new Password for ";
const BOX_PASSWORD_TEXT_ROOT_SGN: &str = "Enter new Root Password:";
const BOX_PASSWORD_TEXT_ROOT_RPT: &str = "Confirm new Root Password:";

// Message box texts
const BOX_MESSAGE_TEXT_USERNAME_INVALID: &str = "Username contains invalid characters\n\
    \n\
    \t\tPlease try again";
const BOX_MESSAGE_TEXT_USERNAME_EMPTY: &str = "Username cannot be empty\n\
    \n\
    \t\tPlease try again";

// General box names
const CANCEL: &str = "cancel";

// Input box names
const FULLNAME: &str = "fullname";
const USERNAME: &str = "username";
const USERGROUPS: &str = "usergroups";

// Password box names 
const PASSWORD_USER_SGN: &str = "password_user_sign";
const PASSWORD_USER_RPT: &str = "password_user_repeat";
const PASSWORD_ROOT_SGN: &str = "password_root_sign";
const PASSWORD_ROOT_RPT: &str = "password_root_repeat";

// Message box names
const USERNAME_INVALID: &str = "username_invalid";
const USERNAME_EMPTY: &str = "username_empty";

// Dialog box options
const CLEAR: &str = "clear";

// Unexpected error messages
const EXP_MBOX: &str = "Could not display message box.";
const EXP_DBOX: &str = "Could not display dialog box.";

// regex
const REGEX_USERNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];

// Exit texts
const EXIT_MSG_CANCEL: &str =   "==> Cancel pressed, exiting..";
const EXIT_MSG_FINISH: &str =   "==> Installation finished! Terminating..";
const EXIT_MSG_CONTACT: &str =  "==> Please contact the owner of this application.";


pub struct App {
    username: String,
}

impl App {
    
    pub fn new() -> App {
        App {
            username: String::new(),
        }    
    }
     
    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Some(USERNAME);
    
        loop {
            match current_box {
                Some(CANCEL) => return Self::cancel(),
                Some(FULLNAME) => current_box = Self::handle_dbox_fullname(self),
                Some(PASSWORD_USER_SGN) => current_box = Self::handle_dbox_password_user_sign(self),
                Some(PASSWORD_USER_RPT) => current_box = Self::handle_dbox_password_user_repeat(self),
                Some(PASSWORD_ROOT_SGN) => current_box = Self::handle_dbox_password_root_sign(self),
                Some(PASSWORD_ROOT_RPT) => current_box = Self::handle_dbox_password_root_repeat(self),
                Some(USERNAME) => current_box = Self::handle_dbox_username(self),
                Some(USERGROUPS) => current_box = Self::handle_dbox_usergroups(self),
                Some(USERNAME_INVALID) => current_box = Self::handle_mbox_username_invalid(),
                Some(USERNAME_EMPTY) => current_box = Self::handle_mbox_username_empty(),
                Some(_) => return Self::unknown_box(),
                None => return Self::exit(),
            };
        }
    }
 
    fn handle_dbox_username(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 10, &BOX_INPUT_TEXT_USERNAME) {
            Some(name) => {
                if name.is_empty() {
                    return Some(USERNAME_EMPTY);
                }
                if RegexSet::new(REGEX_USERNAME).unwrap().is_match(&name) {
                    return Some(USERNAME_INVALID);
                } else {
                    self.username = name;
                    return Some(USERGROUPS);
                }
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_usergroups(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 11, 
            &[BOX_INPUT_TEXT_USERGROUPS_1, &self.username, BOX_INPUT_TEXT_USERGROUPS_3].concat()) {
            Some(_) => {
                return Some(FULLNAME);
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_fullname(&mut self) -> Option<&'static str> {
        match Self::input_box(90, 11, 
            &[BOX_INPUT_TEXT_FULLNAME, &self.username, BOX_TEXT_COLON].concat()) {
            Some(_) => {
                return Some(PASSWORD_USER_SGN);
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_password_user_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &[BOX_PASSWORD_TEXT_USER_SGN, &self.username, BOX_TEXT_COLON].concat()) {
            Some(_) => {
                return Some(PASSWORD_USER_RPT);
            },
            None => return Some(CANCEL),
        };
    }

    fn handle_dbox_password_user_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &[BOX_PASSWORD_TEXT_USER_RPT, &self.username, BOX_TEXT_COLON].concat()) {
            Some(_) => {
                return Some(PASSWORD_ROOT_SGN);
            },
            None => return Some(CANCEL),
        };
    }
    fn handle_dbox_password_root_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &BOX_PASSWORD_TEXT_ROOT_SGN) {
            Some(_) => {
                return Some(PASSWORD_ROOT_RPT);
            },
            None => return Some(CANCEL),
        };
    }
    fn handle_dbox_password_root_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(90, 11, 
            &BOX_PASSWORD_TEXT_ROOT_RPT) {
            Some(_) => {
                return None
            },
            None => return Some(CANCEL),
        };
    }


    fn handle_mbox_username_empty() -> Option<&'static str> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(40);
        dbox.set_height(7);
        dialog::Message::new(BOX_MESSAGE_TEXT_USERNAME_EMPTY)
            .show_with(&dbox)
            .expect(EXP_MBOX);
        Some(USERNAME)
    }


    fn handle_mbox_username_invalid() -> Option<&'static str> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(40);
        dbox.set_height(7);
        dialog::Message::new(BOX_MESSAGE_TEXT_USERNAME_INVALID)
            .show_with(&dbox)
            .expect(EXP_MBOX);
        Some(USERNAME)
    }

    // Input box
    fn input_box(width: u32, height: u32, text: &str) -> Option<String> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(width);
        dbox.set_height(height);

        dialog::Input::new(text) 
        .title(TITLE)
        .show_with(&dbox).expect(EXP_DBOX)
    }

    fn password_box(width: u32, height: u32, text: &str) -> Option<String> {
        let mut dbox = dialog::backends::Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(width);
        dbox.set_height(height);

        dialog::Password::new(text) 
        .title(TITLE)
        .show_with(&dbox).expect(EXP_DBOX)
    }



    fn cancel() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        println!("{}", EXIT_MSG_CANCEL);
        Ok(())
    }

    fn unknown_box() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), EXIT_MSG_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }

    fn exit() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        println!("{}", EXIT_MSG_FINISH);
        Ok(())
    }
}


