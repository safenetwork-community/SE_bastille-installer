use dialog::{backends::Dialog, Choice, DialogBox, Input, Menu, Password, Question};

use crate::app::r#box::Page;
use crate::app::constants::{TITLE, TITRFOQ};

// General empty string
pub const EMPTY: &str = "";

// General box labels
pub const LABEL_BACK: &str = "Back";
pub const LABEL_QUIT: &str = "Quit";

// Input defaults
pub const DEFAULT_FULLNAME: &str = "Useur Bastille";
pub const DEFAULT_HOSTNAME: &str = "reseau-sur";
pub const DEFAULT_USERNAME: &str = "bas";

// general regex
pub const REGEX_HOSTNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];
pub const REGEX_USERNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];

// Errors
pub const ERROR_EMPTY_MENU: &str = "\nExpected at least 20 tokens for --men, have 4.\nUse --help to list options.\n\n\n";

// Dimensions Menu box
const HEIGHT_BOX_MENU: u32 = 20;
const WIDTH_BOX_MENU: u32 = 50;
const HEIGHT_LIST_MENU: u32 = 15;

// Dimensions password box
const HEIGHT_BOX_PASSWORD: u32 = 11;
const WIDTH_BOX_PASSWORD: u32 = 90;

// Dimensions input box
const HEIGHT_BOX_INPUT: u32 = 11;
const WIDTH_BOX_INPUT: u32 = 90;

// Dimensions question box
const DEFAULT_WIDTH_BOX_QUESTION: u32 = 90;
const DEFAULT_HEIGHT_BOX_QUESTION: u32 = 20;

// Dialog box unwrap failure text 
const EXP_DBOX: &str = "Could not display dialog box.";

// Types of boxes
pub enum BoxTypeMenu {
    Default, Main
}

pub trait Default {
    fn get_box_default() -> Dialog; 
}

pub trait BoxHandler {
    fn get_text(&self) -> String; 
    fn handle(&mut self) -> Page; 
}

pub trait PageHandler {
    fn next(&self) -> Page; 
    fn previous(&self) -> Page; 
}

pub struct BoxMenu {}

impl Default for BoxMenu {
    fn get_box_default() -> Dialog {
        let mut dialog = Dialog::new(); 
        dialog.set_cancellabel(LABEL_BACK);
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_MENU);
        dialog.set_height(HEIGHT_BOX_MENU);
        dialog
    }
}

impl BoxMenu {
    pub fn choice(box_type:BoxTypeMenu, text: String, list: Vec<[String; 2]>) -> (Choice, Option<String>) {
        let dbox = match box_type {
            BoxTypeMenu::Main => Self::get_box_main(),
            _ => Self::get_box_default(),
        };
        Menu::new(text, HEIGHT_LIST_MENU, list).show_with(dbox).expect(EXP_DBOX)
    }

    pub fn get_page_from_selection_menu(list: &[(&str, Page)], selection: &str) -> Page {
        match list.into_iter().find(|(x,_)| x == &selection) {
            Some((_,y)) => {
                y.clone()
            },
            _ => Page::NoBoxFound,
        }
    }
    
    pub fn convert_to_dbox_list(list: &[(&str, Page)]) -> Vec<[String; 2]> {
        list.iter().map(|(x,_)| [x.to_string(), String::new()]).collect()
    }

    fn get_box_main() -> Dialog {
        let mut dialog = Dialog::new(); 
        dialog.set_cancellabel(LABEL_QUIT);
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_MENU);
        dialog.set_height(HEIGHT_BOX_MENU);
        dialog
    }
}

pub struct BoxInput {}

impl Default for BoxInput {
    fn get_box_default() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_INPUT);
        dialog.set_height(HEIGHT_BOX_INPUT);
        dialog
    }
}

impl BoxInput {
    pub fn choice(text: String, default: &str, dbox: Option<Dialog>) -> (Choice, Option<String>) {
        match dbox {
            Some(ibox) => Input::new(text).default(default).show_with(&ibox).expect(EXP_DBOX),
            None => Input::new(text).default(default).show_with(Self::get_box_default()).expect(EXP_DBOX),
        }
    }
}

pub struct BoxPassword {}

impl Default for BoxPassword {
    fn get_box_default() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(WIDTH_BOX_PASSWORD);
        dialog.set_height(HEIGHT_BOX_PASSWORD);
        dialog.set_insecure(true);
        dialog
    }
}

impl BoxPassword {
    pub fn choice(text: String, dbox: Option<Dialog>) -> (Choice, Option<String>) {
        match dbox {
            Some(pbox) => Password::new(text).show_with(&pbox).expect(EXP_DBOX),
            None => Password::new(text).show_with(Self::get_box_default()).expect(EXP_DBOX),
        }
    }
}



pub struct BoxQuestion {}

impl Default for BoxQuestion {
    fn get_box_default() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(DEFAULT_WIDTH_BOX_QUESTION);
        dialog.set_height(DEFAULT_HEIGHT_BOX_QUESTION);
        dialog
    }
}

impl BoxQuestion {
    pub fn choice(text: String, dbox: Option<Dialog>) -> Choice {
        match dbox {
            Some(qbox) => Question::new(text).show_with(&qbox).expect(EXP_DBOX),
            None => Question::new(text).show_with(Self::get_box_default()).expect(EXP_DBOX),
        }
    }
}


