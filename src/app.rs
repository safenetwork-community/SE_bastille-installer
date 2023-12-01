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

// General box options
const CLEAR: &str = "clear";

// General box labels
const LABEL_BACK: &str = "Back";
const LABEL_QUIT: &str = "Quit";

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

// dboxes, mboxes and exits
#[derive(Clone)]
enum Page {
    Drive, EmptyHostname, EmptyFullname,
    EmptyMenu, EmptyPasswordRoot, 
    EmptyPasswordUser, EmptyUsername,
    Escape, Finish, Fullname, Hostname,
    InvalidHostname, InvalidUsername,
    KeymapGuest, KeymapHost, 
    KeyvarGuest, KeyvarHost, NoBoxFound, 
    NoMatchPasswordRoot, NoMatchPasswordUser,
    MenuConfig, MenuMain,
    PasswordUserSgn, PasswordUserRpt,
    PasswordRootSgn, PasswordRootRpt,
    QuestionConfig, Quit,
    TimezoneRegion, TimezoneZone,
    UnknownError, Usergroups, Username
}

// default lists
const LIST_MENU_CONFIG: &[(&str, Page)] = &[
    ("enter username", Page::Username),
    ("enter usergroups", Page::Usergroups),
    ("enter fullname", Page::Fullname),
    ("set password of user", Page::PasswordUserSgn),
    ("set password of root", Page::PasswordRootSgn),
    ("select drive", Page::Drive),
    ("select timezone", Page::TimezoneRegion),
    ("enter hostname", Page::Hostname)
];

const LIST_MENU_MAIN: &[(&str, Page)] = &[
    ("Start installation", Page::Username), 
    ("Change keyboard layout", Page::KeymapHost),
    ("Quit", Page::Quit)
];

// Types of boxes
enum BoxTypeMenu {
    Default, Main
}

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

// Input defaults
const DEFAULT_FULLNAME: &str = "Useur Bastille";
const DEFAULT_HOSTNAME: &str = "reseau-sur";
const DEFAULT_USERNAME: &str = "bas";

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

// Errors
const ERROR_EMPTY_MENU: &str = "\nExpected at least 20 tokens for --men, have 4.\nUse --help to list options.\n\n\n";

pub struct App {
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
    error_msg: String,
}

impl App {
    
    pub fn new() -> App {
        App {
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
            error_msg: String::new(),
        }    
    }
     
    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Page::MenuMain;

        loop {
            match current_box {
                Page::MenuConfig => current_box = Self::handle_dbox_menu_config(self),
                Page::MenuMain => current_box = Self::handle_dbox_menu_main(self),
                Page::Quit => return Self::quit(self),
                Page::Escape => return Self::escape(self),
                Page::Drive => current_box = Self::handle_dbox_drive(self),
                Page::Fullname => current_box = Self::handle_dbox_fullname(self),
                Page::Hostname => current_box = Self::handle_dbox_hostname(self),
                Page::KeymapGuest => current_box = Self::handle_dbox_keymap_guest(self),
                Page::KeymapHost => current_box = Self::handle_dbox_keymap_host(self),
                Page::KeyvarGuest => current_box = Self::handle_dbox_keyvar_guest(self),
                Page::KeyvarHost => current_box = Self::handle_dbox_keyvar_host(self),
                Page::PasswordRootSgn => current_box = Self::handle_dbox_password_root_sign(self),
                Page::PasswordRootRpt => current_box = Self::handle_dbox_password_root_repeat(self),
                Page::PasswordUserSgn => current_box = Self::handle_dbox_password_user_sign(self),
                Page::PasswordUserRpt => current_box = Self::handle_dbox_password_user_repeat(self),
                Page::QuestionConfig => current_box = Self::handle_dbox_question_config(self),
                Page::TimezoneRegion => current_box = Self::handle_dbox_timezone_region(self),
                Page::TimezoneZone => current_box = Self::handle_dbox_timezone_zone(self),
                Page::Usergroups => current_box = Self::handle_dbox_usergroups(self),
                Page::Username => current_box = Self::handle_dbox_username(self),            
                Page::EmptyFullname => current_box = Self::message_box(40, 10, BOX_MSG_FULLNAME_EMPTY, Page::Fullname),
                Page::EmptyHostname => current_box = Self::message_box(40, 10, BOX_MSG_HOSTNAME_EMPTY, Page::Hostname),
                Page::EmptyPasswordRoot => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_EMPTY, Page::PasswordRootSgn),
                Page::EmptyPasswordUser => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_EMPTY, Page::PasswordUserSgn),
                Page::EmptyUsername => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_EMPTY, Page::Username),
                Page::InvalidHostname => current_box = Self::message_box(40, 10, BOX_MSG_HOSTNAME_INVALID, Page::Hostname),
                Page::InvalidUsername => current_box = Self::message_box(40, 10, BOX_MSG_USERNAME_INVALID, Page::Username),
                Page::NoMatchPasswordRoot => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_ROOT_NOMATCH, Page::PasswordRootSgn),
                Page::NoMatchPasswordUser => current_box = Self::message_box(40, 10, BOX_MSG_PASSWORD_USER_NOMATCH, Page::PasswordUserSgn),
                Page::EmptyMenu => return Self::empty_menu(),
                Page::UnknownError => return Self::unknown_error(self),
                Page::Finish => return Self::finish(self),
                _ => return Self::box_not_found(),
            };
        }
    }
     
    fn handle_dbox_menu_main(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Main, &TEXT_BOX_MENU_MAIN, 
            Self::get_list_menu(&LIST_MENU_MAIN)) {
            (Choice::Yes, Some(choice)) => {
                Self::get_menu_choice(&LIST_MENU_MAIN, &choice)
            },
            (Choice::Escape, Some(msg_txt)) => {
                match msg_txt.as_str() {
                    EMPTY => Page::Escape,
                    ERROR_EMPTY_MENU => Page::EmptyMenu,
                    _ => {
                        self.error_msg = msg_txt;
                        Page::UnknownError
                    },
                }
            },
            (Choice::Cancel, _) => Page::Quit,
            _ => Page::NoBoxFound,
        }
    }
     
    fn handle_dbox_username(&mut self) -> Page {
        let mut dbox = Self::get_dbox_input();
        dbox.set_cancellabel("Back");
        match Self::input_box(&TEXT_BOX_INPUT_USERNAME, DEFAULT_USERNAME,
        Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_USERNAME).unwrap().is_match(&input_text) {
                        self.username = input_text;
                        Page::Usergroups
                    }
                    else { 
                        Page::InvalidUsername
                    }
                }
                else { 
                    Page::EmptyUsername
                }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::MenuMain,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_usergroups(&mut self) -> Page {
        let mut dbox = Self::get_dbox_input();
        dbox.set_cancellabel("Back");
        match Self::input_box(&[TEXT_BOX_INPUT_USERGROUPS_1, 
            &self.username, TEXT_BOX_INPUT_USERGROUPS_3].concat(),
        EMPTY, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                self.usergroups = input_text;
                Page::Fullname
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Username,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_fullname(&mut self) -> Page {
        let mut dbox = Self::get_dbox_input();
        dbox.set_cancellabel("Back");
        match Self::input_box(&[TEXT_BOX_INPUT_FULLNAME, 
            &self.username, TEXT_COLON].concat(), DEFAULT_FULLNAME, 
        Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    self.fullname = input_text;
                    Page::PasswordUserSgn
                }
                else { Page::EmptyFullname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Usergroups,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_password_user_sign(&mut self) -> Page {
        let mut dbox = Self::get_dbox_password();
        dbox.set_cancellabel("Back");
        match Self::password_box(&[TEXT_BOX_PASSWORD_USER_SGN, 
            &self.username, TEXT_COLON].concat(),
        Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    self.password_user = password;
                    Page::PasswordUserRpt
                }
                else { Page::EmptyPasswordUser }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Fullname,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_password_user_repeat(&mut self) -> Page {
        let mut dbox = Self::get_dbox_password();
        dbox.set_cancellabel("Back");
        match Self::password_box(&[TEXT_BOX_PASSWORD_USER_RPT, 
            &self.username, TEXT_COLON].concat(),
        Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if self.password_user.eq(&password) {
                    Page::PasswordRootSgn
                }
                else { Page::NoMatchPasswordUser }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::PasswordUserSgn,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_password_root_sign(&mut self) -> Page {
        let mut dbox = Self::get_dbox_password();
        dbox.set_cancellabel("Back");
        match Self::password_box(&TEXT_BOX_PASSWORD_ROOT_SGN,
        Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    self.password_root = password;
                    Page::PasswordRootRpt
                }
                else { Page::EmptyPasswordRoot }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::PasswordUserSgn,
            _ => Page::NoBoxFound,
        }
    }
    
    fn handle_dbox_password_root_repeat(&mut self) -> Page {
        let mut dbox = Self::get_dbox_password();
        dbox.set_cancellabel("Back");
        match Self::password_box(&TEXT_BOX_PASSWORD_ROOT_RPT,
        Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if self.password_user.eq(&password) {
                    Page::Drive
                }
                else { Page::NoMatchPasswordRoot }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::PasswordRootSgn,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_drive(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_DRIVE, 
            Self::drivelist_from_lsblk()) {
            (Choice::Yes, Some(drive)) => {
                self.drive = drive;
                Page::TimezoneRegion
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::PasswordRootSgn,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_timezone_region(&mut self) -> Page { 
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_TIMEZONE_REGION, 
            Self::get_list_timeregion()) {
            (Choice::Yes, Some(region)) => {
                self.timezone_path = Path::new(PATH_ZONEINFO).join(region);
                Page::TimezoneZone
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Drive,
            _ => Page::NoBoxFound,
        }
    }


    fn handle_dbox_timezone_zone(&mut self) -> Page {
        let mut dbox = Self::get_dbox_menu();
        dbox.set_cancellabel("Back");
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_TIMEZONE_ZONE, 
            Self::get_list_timezone(self.timezone_path.as_path())) {
            (Choice::Yes, Some(zone)) => {
                self.timezone_zone = zone;
                Page::KeymapGuest
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Drive,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_keymap_guest(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_KEYMAP_GUEST, 
            Self::get_list_keymap()) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap_guest = keymap.clone();
                self.keyvar_path = Path::new(PATH_BKEYMAP).join(keymap);
                Page::KeyvarGuest
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::TimezoneRegion,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_keyvar_guest(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_KEYVAR_GUEST, 
            Self::get_list_keyvars(self.keyvar_path.as_path())) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar_guest = keyvar;
                Page::Hostname
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapGuest,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_hostname(&mut self) -> Page {
        let mut dbox = Self::get_dbox_input();
        dbox.set_cancellabel("Back");
        match Self::input_box(&TEXT_BOX_INPUT_HOSTNAME, DEFAULT_HOSTNAME,
        Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_HOSTNAME).unwrap().is_match(&input_text) {
                        self.hostname = input_text;
                        Page::QuestionConfig
                    }
                    else { Page::InvalidHostname }
                }
                else { Page::EmptyHostname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapGuest,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_question_config(&mut self) -> Page {
        match Self::question_box(&TEXT_BOX_QUESTION_CONFIG, None) {
            Choice::Yes => Page::Finish,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::NoBoxFound,
        }
    }
   
    fn handle_dbox_menu_config(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_CONFIG, 
            Self::get_list_menu(&LIST_MENU_CONFIG)) {
            (Choice::Yes, Some(choice)) => {
                Self::get_menu_choice(&LIST_MENU_CONFIG, &choice)
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapGuest,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_keymap_host(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_KEYMAP_HOST, 
            Self::get_list_keymap()) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap_host = keymap.clone();
                self.keyvar_path = Path::new(PATH_BKEYMAP).join(keymap);
                Page::KeyvarHost
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Quit,
            _ => Page::NoBoxFound,
        }
    }

    fn handle_dbox_keyvar_host(&mut self) -> Page {
        match Self::menu_box(BoxTypeMenu::Default, &TEXT_BOX_MENU_KEYVAR_HOST, 
            Self::get_list_keyvars(self.keyvar_path.as_path())) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar_host = keyvar;
                self.setup_keymap();
                Page::Username
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapHost,
            _ => Page::NoBoxFound,
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
    fn message_box(width: u32, height: u32, text :&str, return_box :Page) -> Page {
        let mut dbox = Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(width);
        dbox.set_height(height);
        dialog::Message::new(text)
            .show_with(&dbox)
            .expect(EXP_MBOX);
        return_box
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

    fn menu_box(box_type: BoxTypeMenu, text: &str, list: Vec<[String; 2]>) -> (Choice, Option<String>) { 
        let dbox = match box_type {
            BoxTypeMenu::Main => Self::get_dbox_menu_main(),
            _ => Self::get_dbox_menu(),
        }; 
        Menu::new(text, HEIGHT_LIST_MENU, list).show_with(dbox).expect(EXP_DBOX)
    }    

    fn get_dbox_menu_main() -> Dialog {
        let mut dialog = Dialog::new(); 
        dialog.set_cancellabel(LABEL_QUIT);
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_MENU);
        dialog.set_height(HEIGHT_BOX_MENU);
        dialog
    }

    fn get_dbox_menu() -> Dialog {
        let mut dialog = Dialog::new(); 
        dialog.set_cancellabel(LABEL_BACK);
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_MENU);
        dialog.set_height(HEIGHT_BOX_MENU);
        dialog
    }
    
    fn quit(&mut self) -> Result<(), Error> {      
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
        println!("==> {}", msg);
        Ok(())
    }

    fn box_not_found() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }
 
    fn empty_menu() -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}\n{}", Error::new(ErrorKind::EmptyMenu()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::EmptyMenu()))    
    }

    fn unknown_error(&mut self) -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}\n{}", Error::new(ErrorKind::UnknownError(self.error_msg.clone())), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::UnknownError(self.error_msg.clone())))    
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

    fn get_list_menu(list: &[(&str, Page)]) -> Vec<[String; 2]> {
        list.iter().map(|(x,_)| [x.to_string(), String::new()]).collect()
    }

    fn get_menu_choice(list: &[(&str, Page)], choice: &str) -> Page {
        match list.into_iter().find(|(x,_)| x == &choice) {
            Some((_,y)) => {
                y.clone()
            },
            _ => Page::NoBoxFound,
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
