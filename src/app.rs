mod constants;

use const_format::concatcp;
use dialog::{backends::Dialog, DialogBox, Input, Menu, Password, Choice, Question};
use regex::RegexSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::Error as Error; 
use crate::error::ErrorKind as ErrorKind; 

const TITRFOQ: &str = concatcp!("Èstalxr d sistêmbêstur d Bastij", " ", constants::VERSION);
const TITLE: &str = concatcp!(constants::OS, " ", constants::VERSION);

// General empty string
const EMPTY: &str = "";

// General box names
const CANCEL: &str = "cancel";
const ESCAPE: &str = "escape";
const EMPTY_MENU: &str = "empty_menu";
const UNKNOWN: &str = "unknown";

// General box options
const CLEAR: &str = "clear";

// General box texts
const TEXT_COLON: &str = ":";

// Input box texts
const TEXT_BOX_INPUT_HOSTNAME: &str = "Enter desired hostname for this system:";
const TEXT_BOX_INPUT_FULLNAME: &str = "Enter desired Full Name for ";
const TEXT_BOX_INPUT_USERNAME: &str = "Enter the username you want: \n\
    (usernames must be all lowercase and first character may not be a number)"; 
const TEXT_BOX_INPUT_USERGROUPS_1: &str = "Enter additional groups besides the default groups which are\n\
    --> wheel,sys,audio,input,video,storage,lp,network,users,power <--\n\
    for user \'";
const TEXT_BOX_INPUT_USERGROUPS_3: &str = "\' in a comma seperated list:"; 

// Menu box texts
const TEXT_BOX_MENU_CONFIG: &str = "What would you like to reconfigure:";
const TEXT_BOX_MENU_DRIVE: &str = "Choose your SDCard/eMMC/USB - Be sure the correct drive is selected!\n\
    WARNING! This WILL destroy the data on it!";
const TEXT_BOX_MENU_KEYMAP_GUEST: &str = "Choose your desired keyboard layout:";
const TEXT_BOX_MENU_KEYMAP_HOST: &str = "Choose a keyboard layout for this installer:";
const TEXT_BOX_MENU_KEYVAR_GUEST: &str = "Choose your desired keyboard variant:";
const TEXT_BOX_MENU_KEYVAR_HOST: &str = "Choose a keyboard variant for this installer:";
const TEXT_BOX_MENU_MAIN: &str = "What would you like to do:";
const TEXT_BOX_MENU_TIMEZONE_REGION: &str = "Choose your timezone:";
const TEXT_BOX_MENU_TIMEZONE_ZONE: &str = "Choose your timezone:";

// Password box texts
const TEXT_BOX_PASSWORD_USER_SGN: &str = "Enter new Password for ";
const TEXT_BOX_PASSWORD_USER_RPT: &str = "Confirm new Password for ";
const TEXT_BOX_PASSWORD_ROOT_SGN: &str = "Enter new Root Password:";
const TEXT_BOX_PASSWORD_ROOT_RPT: &str = "Confirm new Root Password:";

// Question box texts
const TEXT_BOX_QUESTION_CONFIG: &str = "Is the below information correct:";

// Message box texts
const BOX_MSG_FULLNAME_EMPTY: &str = concatcp!("Fullname", constants::BOX_MSG_EMPTY);
const BOX_MSG_HOSTNAME_EMPTY: &str = concatcp!("Hostname", constants::BOX_MSG_EMPTY);
const BOX_MSG_HOSTNAME_INVALID: &str = concatcp!("Hostname", constants::BOX_MSG_INVALID);
const BOX_MSG_USERNAME_EMPTY: &str = concatcp!("Username", constants::BOX_MSG_EMPTY);
const BOX_MSG_USERNAME_INVALID: &str = concatcp!("Username", constants::BOX_MSG_INVALID);
const BOX_MSG_PASSWORD_ROOT_EMPTY: &str = concatcp!("Root password", constants::BOX_MSG_EMPTY);
const BOX_MSG_PASSWORD_ROOT_NOMATCH: &str = concatcp!("Root passwords", constants::BOX_MSG_DONOTMATCH);
const BOX_MSG_PASSWORD_USER_EMPTY: &str = concatcp!("User password", constants::BOX_MSG_EMPTY);
const BOX_MSG_PASSWORD_USER_NOMATCH: &str = concatcp!("User passwords", constants::BOX_MSG_DONOTMATCH);

// Input box names
const FULLNAME: &str = "enter fullname";
const HOSTNAME: &str = "enter hostname";
const USERNAME: &str = "enter username";
const USERGROUPS: &str = "enter usergroups";

// Menu box names
const DRIVE: &str = "select drive";
const KEYMAP_HOST: &str = "keymap_host";
const KEYVAR_HOST: &str = "select keyvar";
const KEYMAP_GUEST: &str = "keymap_guest";
const KEYVAR_GUEST: &str = "keyvar_guest";
const MENU_CONFIG: &str = "menu_config";
const MENU_MAIN: &str = "menu_main";
const TIMEZONE_REGION: &str = "select timezone";
const TIMEZONE_ZONE: &str = "timezone_zone";

// Password box names 
const PASSWORD_USER_SGN: &str = "select password user";
const PASSWORD_USER_RPT: &str = "password_user_repeat";
const PASSWORD_ROOT_SGN: &str = "select password root";
const PASSWORD_ROOT_RPT: &str = "password_root_repeat";

// Question box names
const QUESTION_CONFIG: &str = "question_config";

// default lists
const LIST_MENU_CONFIG: [[&str; 2]; 9] = [
    ["select keymap", KEYMAP_HOST], 
    ["enter username", USERNAME],
    ["enter usergroups", USERGROUPS],
    ["enter fullname", FULLNAME],
    ["set password of user", PASSWORD_USER_SGN],
    ["set password of root", PASSWORD_ROOT_SGN],
    ["select drive", DRIVE],
    ["select timezone", TIMEZONE_REGION],
    ["enter hostname", HOSTNAME],
];

const LIST_MENU_MAIN: [[&str; 2]; 3] = [
    ["Start installation", USERNAME], 
    ["Change keyboard layout", KEYMAP_HOST],
    ["Quit", CANCEL]
];

// Message box names
const FULLNAME_EMPTY: &str = "fullname_empty";
const HOSTNAME_INVALID: &str = "hostname_invalid";
const HOSTNAME_EMPTY: &str = "hostname_empty";
const USERNAME_INVALID: &str = "username_invalid";
const USERNAME_EMPTY: &str = "username_empty";
const PASSWORD_USER_EMPTY: &str = "user_password_empty";
const PASSWORD_USER_NOMATCH: &str = "user_password_nomatch";
const PASSWORD_ROOT_EMPTY: &str = "root_password_empty";
const PASSWORD_ROOT_NOMATCH: &str = "root_password_nomatch";

// Input defaults
const DEFAULT_FULLNAME: &str = "Useur Bastille";
const DEFAULT_HOSTNAME: &str = "reseau-sur";
const DEFAULT_USERNAME: &str = "bas";

// Dimensions Menu box
const HEIGHT_BOX_MENU: u32 = 20;
const WIDTH_BOX_MENU: u32 = 50;
const HEIGHT_LIST_MENU: u32 = 15;

// Dimensions password box
const HEIGHT_BOX_PASSWORD: u32 = 11;
const WIDTH_BOX_PASSWORD: u32 = 90;

// Dimensions question box
const HEIGHT_BOX_QUESTION: u32 = 20;
const WIDTH_BOX_QUESTION: u32 = 90;

// Dimensions input box
const HEIGHT_BOX_INPUT: u32 = 11;
const WIDTH_BOX_INPUT: u32 = 90;

// Unexpected error messages
const EXP_MBOX: &str = "Could not display message box.";
const EXP_DBOX: &str = "Could not display dialog box.";

// general regex
const REGEX_HOSTNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];
const REGEX_USERNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];

// find regex
const REGEX_FIND_DIRS_ALL: [&str; 6] = [".","-regex",r"\.\/[^\.].*","-prune","-type","d"];
const REGEX_FIND_DIRS_CAP: [&str; 6] = [".","-regex",r"\.\/[A-Z].*","-prune","-type","d"];
const REGEX_FIND_FILES: [&str; 6] = [".","-regex",r"\.\/[^\.].*","-prune","-type","f"];

// sed regex
const SED_FIND_DEFAULT: &str = r"s/\.\///";
const SED_FIND_FILE_EXTENSIONS: &str = r"s/\.\/\([^\.]*\).*/\1/";

// Exit texts
const MSG_EXIT_ESCAPE: &str =   "==> Escape pressed, exiting..";
const MSG_EXIT_QUIT: &str =     "==> Quit pressed, exiting..";
const MSG_EXIT_FINISH: &str =   "==> Installation finished! Terminating..";
const MSG_EXIT_CONTACT: &str =  "==> Please contact the owner of this application.";

// Paths
const PATH_BKEYMAP: &str = "/usr/share/bkeymaps";  
const PATH_ZONEINFO: &str = "/usr/share/zoneinfo";  

pub struct App {
    choice: Choice,
    username: String,
    usergroups: String,
    fullname: String,
    keymap_guest: String,
    keymap_host: String,
    keyvar_guest: String,
    keyvar_host: String,
    keyvar_path: PathBuf,
    password_user: String,
    password_root: String,
    drive: String,
    timezone_path: PathBuf,
    timezone_zone: String,
    hostname: String,
}

impl App {
    
    pub fn new() -> App {
        App {
            choice: Choice::Escape,
            username: String::new(),
            fullname: String::new(),
            keymap_guest: String::new(),
            keymap_host: String::new(),
            keyvar_guest: String::new(),
            keyvar_host: String::new(),
            keyvar_path: PathBuf::new(),
            password_user: String::new(),
            password_root: String::new(),
            drive: String::new(),
            timezone_path: PathBuf::new(),
            timezone_zone: String::new(),
            usergroups: String::new(),
            hostname: String::new(),
        }    
    }
     
    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Some(MENU_MAIN);
        let mut menu: Option<String>;

        loop {
            match current_box {
                Some(CANCEL) => return Self::cancel(self),
                Some(ESCAPE) => return Self::escape(self),
                Some(EMPTY_MENU) => return Self::empty_menu(),
                Some(DRIVE) => current_box = Self::handle_dbox_drive(self),
                Some(FULLNAME) => current_box = Self::handle_dbox_fullname(self),
                Some(FULLNAME_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_FULLNAME_EMPTY, FULLNAME),
                Some(HOSTNAME) => current_box = Self::handle_dbox_hostname(self),
                Some(HOSTNAME_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_HOSTNAME_EMPTY, HOSTNAME_EMPTY),
                Some(HOSTNAME_INVALID) => current_box = Self::message_box(40, 10, BOX_MSG_HOSTNAME_INVALID, HOSTNAME_INVALID),
                Some(KEYMAP_GUEST) => current_box = Self::handle_dbox_keymap_guest(self),
                Some(KEYMAP_HOST) => current_box = Self::handle_dbox_keymap_host(self),
                Some(KEYVAR_GUEST) => current_box = Self::handle_dbox_keyvar_guest(self),
                Some(KEYVAR_HOST) => current_box = Self::handle_dbox_keyvar_host(self),
                Some(MENU_CONFIG) => {                    
                    menu = Self::handle_dbox_menu_config(self);
                    current_box = menu.as_deref();
                },
                Some(MENU_MAIN) => {
                    menu = Self::handle_dbox_menu_main(self);
                    current_box = menu.as_deref();
                },
                Some(PASSWORD_ROOT_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_EMPTY, PASSWORD_ROOT_SGN),
                Some(PASSWORD_ROOT_NOMATCH) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_NOMATCH, PASSWORD_ROOT_SGN),
                Some(PASSWORD_ROOT_SGN) => current_box = Self::handle_dbox_password_root_sign(self),
                Some(PASSWORD_ROOT_RPT) => current_box = Self::handle_dbox_password_root_repeat(self),
                Some(PASSWORD_USER_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_EMPTY, PASSWORD_USER_SGN),
                Some(PASSWORD_USER_NOMATCH) => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_NOMATCH, PASSWORD_USER_SGN),
                Some(PASSWORD_USER_SGN) => current_box = Self::handle_dbox_password_user_sign(self),
                Some(PASSWORD_USER_RPT) => current_box = Self::handle_dbox_password_user_repeat(self),
                Some(QUESTION_CONFIG) => current_box = Self::handle_dbox_question_config(self),
                Some(TIMEZONE_REGION) => current_box = Self::handle_dbox_timezone_region(self),
                Some(TIMEZONE_ZONE) => current_box = Self::handle_dbox_timezone_zone(self),
                Some(USERGROUPS) => current_box = Self::handle_dbox_usergroups(self),
                Some(USERNAME) => current_box = Self::handle_dbox_username(self),
                Some(USERNAME_EMPTY) => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_EMPTY, USERNAME),
                Some(USERNAME_INVALID) => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_INVALID, USERNAME),
                Some(_) => return Self::unknown_box(),
                None => return Self::finish(self),
            };
        }
    }
     
    fn handle_dbox_keymap_host(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_extrabutton("Skip");
        dbox.set_cancellabel("Quit");
        match Self::menu_box(&TEXT_BOX_MENU_KEYMAP_HOST, 
            Self::get_list_keymap(), 
        Some(dbox)) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap_host = keymap.clone();
                self.keyvar_path = Path::new(PATH_BKEYMAP).join(keymap);
                Some(KEYVAR_HOST)
            },
            (Choice::Extra, _) => Some(USERNAME),
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(CANCEL),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_keyvar_host(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::menu_box(&TEXT_BOX_MENU_KEYVAR_HOST, 
            Self::get_list_keyvars(self.keyvar_path.as_path()), 
        Some(dbox)) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar_host = keyvar;
                self.setup_keymap();
                return Some(USERNAME);
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(KEYMAP_HOST),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_username(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::input_box(&TEXT_BOX_INPUT_USERNAME, DEFAULT_USERNAME,
        Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_USERNAME).unwrap().is_match(&input_text) {
                        self.username = input_text;
                        Some(USERGROUPS)
                    }
                    else { 
                        Some(USERNAME_INVALID)
                    }
                }
                else { 
                    Some(USERNAME_EMPTY)
                }
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(MENU_MAIN),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_usergroups(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::input_box(&[TEXT_BOX_INPUT_USERGROUPS_1, 
            &self.username, TEXT_BOX_INPUT_USERGROUPS_3].concat(),
        EMPTY, None) {
            (Choice::Yes, Some(input_text)) => {
                self.usergroups = input_text;
                return Some(FULLNAME);
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(USERNAME),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_fullname(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::input_box(&[TEXT_BOX_INPUT_FULLNAME, 
            &self.username, TEXT_COLON].concat(), DEFAULT_FULLNAME, 
        None) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    self.fullname = input_text;
                    return Some(PASSWORD_USER_SGN);
                }
                else { return Some(FULLNAME_EMPTY) }
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(USERGROUPS),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_password_user_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(&[TEXT_BOX_PASSWORD_USER_SGN, 
            &self.username, TEXT_COLON].concat(),
        None) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    self.password_user = password;
                    return Some(PASSWORD_USER_RPT);
                }
                else { return Some(PASSWORD_USER_EMPTY); }
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(FULLNAME),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_password_user_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(&[TEXT_BOX_PASSWORD_USER_RPT, 
            &self.username, TEXT_COLON].concat(),
        None) {
            (Choice::Yes, Some(password)) => {
                if self.password_user.eq(&password) {
                    return Some(PASSWORD_ROOT_SGN);
                }
                else { return Some(PASSWORD_USER_NOMATCH); }
            },
            _ => Some(CANCEL),
        }
    }

    fn handle_dbox_password_root_sign(&mut self) -> Option<&'static str> {
        match Self::password_box(&TEXT_BOX_PASSWORD_ROOT_SGN,
        None) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    self.password_root = password;
                    return Some(PASSWORD_ROOT_RPT);
                }
                else { return Some(PASSWORD_ROOT_EMPTY); }
            },
            _ => Some(CANCEL),
        }
    }
    
    fn handle_dbox_password_root_repeat(&mut self) -> Option<&'static str> {
        match Self::password_box(&TEXT_BOX_PASSWORD_ROOT_RPT,
        None) {
            (Choice::Yes, Some(password)) => {
                if self.password_user.eq(&password) {
                    return Some(DRIVE);
                }
                else { return Some(PASSWORD_ROOT_NOMATCH) }
            },
            _ => Some(CANCEL),
        }
    }

    fn handle_dbox_drive(&mut self) -> Option<&'static str> {
        match Self::menu_box(&TEXT_BOX_MENU_DRIVE, 
            Self::drivelist_from_lsblk(),
        None) {
            (Choice::Yes, Some(drive)) => {
                self.drive = drive;
                Some(TIMEZONE_REGION)
            },
            (choice, _) => {
                self.choice = choice;
                Some(CANCEL)
            },
        }
    }

    fn handle_dbox_timezone_region(&mut self) -> Option<&'static str> {
        match Self::menu_box(&TEXT_BOX_MENU_TIMEZONE_REGION, 
            Self::get_list_timeregion(),
        None) {
            (Choice::Yes, Some(region)) => {
                self.timezone_path = Path::new(PATH_ZONEINFO).join(region);
                Some(TIMEZONE_ZONE)
            },
            (Choice::Cancel, _) => {
                Some(CANCEL)
            },
            _ => Some(EMPTY_MENU),
        }
    }


    fn handle_dbox_timezone_zone(&mut self) -> Option<&'static str> {
        match Self::menu_box(&TEXT_BOX_MENU_TIMEZONE_ZONE, 
            Self::get_list_timezone(self.timezone_path.as_path()), 
        None) {
            (Choice::Yes, Some(zone)) => {
                self.timezone_zone = zone;
                return Some(KEYMAP_GUEST);
            },
            _ => Some(CANCEL),
        }
    }

    fn handle_dbox_keymap_guest(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back"); 
        match Self::menu_box(&TEXT_BOX_MENU_KEYMAP_GUEST, 
            Self::get_list_keymap(), 
        Some(dbox)) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap_guest = keymap.clone();
                self.keyvar_path = Path::new(PATH_BKEYMAP).join(keymap);
                return Some(KEYVAR_GUEST);
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(TIMEZONE_REGION),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_keyvar_guest(&mut self) -> Option<&'static str> {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::menu_box(&TEXT_BOX_MENU_KEYVAR_GUEST, 
            Self::get_list_keyvars(self.keyvar_path.as_path()), 
        Some(dbox)) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar_guest = keyvar;
                return Some(HOSTNAME);
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(KEYMAP_GUEST),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_hostname(&mut self) -> Option<&'static str> {
        match Self::input_box(&TEXT_BOX_INPUT_HOSTNAME, DEFAULT_HOSTNAME,
        None) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_HOSTNAME).unwrap().is_match(&input_text) {
                        self.hostname = input_text;
                        return Some(QUESTION_CONFIG);
                    }
                    else { return Some(HOSTNAME_INVALID) }
                }
                else { return Some(HOSTNAME_EMPTY) }
            },
            (Choice::Escape, _) => Some(ESCAPE),
            (Choice::Cancel, _) => Some(KEYMAP_GUEST),
            _ => Some(UNKNOWN),
        }
    }

    fn handle_dbox_question_config(&mut self) -> Option<&'static str> {
        match Self::question_box(&TEXT_BOX_QUESTION_CONFIG, None) {
            Choice::Yes => None,
            Choice::No => Some(MENU_CONFIG),
            Choice::Escape => Some(ESCAPE),
            _ => Some(UNKNOWN),
        }
    }
   
    fn handle_dbox_menu_config(&mut self) -> Option<String> {
        match Self::menu_box(&TEXT_BOX_MENU_CONFIG, 
            Self::get_list_menu(&LIST_MENU_CONFIG), None) {
            (Choice::Yes, Some(choice)) => {
                Self::get_menu_choice(&LIST_MENU_CONFIG, choice)
            },
            (Choice::Escape, _) => Some(String::from(ESCAPE)),
            (Choice::Cancel, _) => Some(String::from(KEYMAP_GUEST)),
            _ => Some(String::from(UNKNOWN)),
        }
    }

    fn handle_dbox_menu_main(&mut self) -> Option<String> {
        match Self::menu_box(&TEXT_BOX_MENU_MAIN, 
            Self::get_list_menu(&LIST_MENU_MAIN), None) {
            (Choice::Yes, Some(choice)) => {
                Self::get_menu_choice(&LIST_MENU_MAIN, choice)
            },
            (Choice::Escape, _) => Some(String::from(ESCAPE)),
            (Choice::Cancel, _) => Some(String::from(CANCEL)),
            _ => Some(String::from(UNKNOWN)),
        }
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
        let mut dbox = Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(width);
        dbox.set_height(height);
        dialog::Message::new(text)
            .show_with(&dbox)
            .expect(EXP_MBOX);
        Some(return_box)
    }
 
    // Input box
    fn input_box(text: &str, default: &str, dbox: Option<Dialog>) -> (Choice, Option<String>) {
        match dbox {
            Some(ibox) => Input::new(text).default(default).show_with(&ibox).expect(EXP_DBOX),
            None => Input::new(text).default(default).show_with(Self::get_dbox_input()).expect(EXP_DBOX),
        }
    }

    fn get_dbox_input() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_INPUT);
        dialog.set_height(HEIGHT_BOX_INPUT);
        dialog
    }

    fn password_box(text: &str, dbox: Option<Dialog>) -> (Choice, Option<String>) {
        match dbox {
            Some(pbox) => Password::new(text).show_with(&pbox).expect(EXP_DBOX),
            None => Password::new(text).show_with(Self::get_dbox_password()).expect(EXP_DBOX),
        }
    }

    fn get_dbox_password() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_PASSWORD);
        dialog.set_height(HEIGHT_BOX_PASSWORD);
        dialog.set_insecure(true);
        dialog
    }

    fn question_box(text: &str, dbox: Option<Dialog>) -> Choice {
        match dbox {
            Some(qbox) => Question::new(text).show_with(&qbox).expect(EXP_DBOX),
            None => Question::new(text).show_with(Self::get_dbox_question()).expect(EXP_DBOX),
        }
    }

    fn get_dbox_question() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_QUESTION);
        dialog.set_height(HEIGHT_BOX_QUESTION);
        dialog
    }

    fn menu_box(text: &str, list: Vec<[String; 2]>, dbox: Option<Dialog>) -> (Choice, Option<String>) { 
        match dbox {
            Some(mbox) => Menu::new(text, HEIGHT_LIST_MENU, list).show_with(&mbox).expect(EXP_DBOX),
            None => Menu::new(text, HEIGHT_LIST_MENU, list).show_with(Self::get_dbox_menu()).expect(EXP_DBOX),
        }
    }    

    fn get_dbox_menu() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_MENU);
        dialog.set_height(HEIGHT_BOX_MENU);
        dialog
    }
    
    fn cancel(&mut self) -> Result<(), Error> {      
        Self::exit(self, MSG_EXIT_QUIT)
    }

    fn escape(&mut self) -> Result<(), Error> {     
        Self::exit(self, MSG_EXIT_ESCAPE)
    }

    fn finish(&mut self) -> Result<(), Error> {
        Self::exit(self, MSG_EXIT_FINISH)
    }

    fn exit(&mut self, msg: &str) -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("{}", msg);
        println!("{:?}", self.choice);
        Ok(())
    }

    fn empty_menu() -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}\n{}", Error::new(ErrorKind::EmptyMenu()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::EmptyMenu()))    
    }

    fn unknown_box() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }
     
    fn drivelist_from_lsblk() -> Vec<[String; 2]> {
        let args: [&str; 3] = ["-dn","-o", "NAME"];
        let ls_output = Command::new("lsblk")
        .args(args)
        .output()
        .expect(format!("Failed to execute lsblk {:?}", args).as_str());
 
        let mut list: Vec<[String; 2]> = Vec::new(); 
        let output = String::from_utf8_lossy(&ls_output.stdout);
        for line in output.lines() {
           list.push([String::from(line),String::new()]); 
        } 
        return list
    }

    fn get_list_keyvars(path: &Path) -> Vec<[String; 2]> {
        Self::list_from_find(path, 
            REGEX_FIND_FILES.to_vec(), SED_FIND_FILE_EXTENSIONS)
    }

    fn get_list_keymap() -> Vec<[String; 2]> {
        Self::list_from_find(Path::new(PATH_BKEYMAP), 
            REGEX_FIND_DIRS_ALL.to_vec(), SED_FIND_DEFAULT)
    }
       
    fn get_list_timeregion() -> Vec<[String; 2]> {
        Self::list_from_find(Path::new(PATH_ZONEINFO), 
            REGEX_FIND_DIRS_CAP.to_vec(), SED_FIND_DEFAULT)
    }

    fn get_list_timezone(path: &Path) -> Vec<[String; 2]> {
        Self::list_from_find(path, 
            REGEX_FIND_FILES.to_vec(), SED_FIND_DEFAULT)
    }

    fn get_list_menu(list: &[[&str; 2]]) -> Vec<[String; 2]> {
        list.iter().map(|[x,_]| [x.to_string(), String::new()]).collect()
    }

    fn get_menu_choice(list: &[[&str; 2]], choice: String) -> Option<String> {
        match list.iter().find(|[x,_]| x == &choice) {
            Some([_,y]) => Some(y.to_string()),
            _ => None,
        }
    }

    fn list_from_find(path: &Path, regex_find: Vec<&str>, regex_sed: &str) -> Vec<[String; 2]> {
        let process_find = Command::new("find")
        .current_dir(path.to_str().unwrap())
        .args(regex_find)
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("Failed to execute find {}", path.display()).as_str());

        let process_sed = Command::new("sed")
        .arg(regex_sed)
        .stdin(Stdio::from(process_find.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("Failed to execute sed {}", path.display()).as_str());

        let output = process_sed
            .wait_with_output()
            .expect("Failed to wait on sed process");        

        let output_string = String::from_utf8_lossy(&output.stdout);
        let mut dirs: Vec<[String; 2]> = Vec::new(); 
        for line in output_string.lines() {
           dirs.push([String::from(line),String::new()]); 
        } 
        dirs
    }
}
