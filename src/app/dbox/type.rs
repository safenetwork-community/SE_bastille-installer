use dialog::{
    backends::Dialog, 
    Choice, DialogBox, 
    Gauge, Input, Menu,
    Message, Password, 
    Question
};

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
// const HEIGHT_LIST_FORM: u32 = 15;

// Dimensions password box
const HEIGHT_BOX_PASSWORD: u32 = 11;
const WIDTH_BOX_PASSWORD: u32 = 90;

// Dimensions input box
const HEIGHT_BOX_INPUT: u32 = 11;
const WIDTH_BOX_INPUT: u32 = 90;

// Dimensions question box
const HEIGHT_BOX_QUESTION: u32 = 20;
const WIDTH_BOX_QUESTION: u32 = 90;

// Dimensions mixed gauge box
const HEIGHT_BOX_MIXED_GAUGE: u32 = 11;
const WIDTH_BOX_MIXED_GAUGE: u32 = 90;

// Dimensions message box
// const HEIGHT_BOX_MESSAGE: u32 = 40;
// const WIDTH_BOX_MESSAGE: u32 = 10;



// Dialog box unwrap failure text 
const EXP_DBOX: &str = "Could not display dialog box.";

// dboxes, mboxes and exits
#[derive(Clone)]
pub enum Page {
    EmptyHostname, EmptyFullname,
    EmptyMenu, EmptyPasswordRoot, 
    EmptyPasswordUser, EmptyUsername,
    Escape, Finish,
    InputFullname, InputHostname,
    InputUsergroups, InputUsername,
    InvalidHostname, InvalidUsername,
    MenuConfig, MenuDevice, MenuDrive, 
    MenuKeymapGuest, MenuKeymapHost, 
    MenuKeyvarGuest, MenuKeyvarHost, 
    MenuMain, MenuOperatingSystem, 
    MenuTimezoneRegion, MenuTimezoneZone, 
    GaugeInstallation, NoBoxFound, 
    NoMatchPasswordRoot, NoMatchPasswordUser,
    PasswordUserSgn, PasswordUserRpt,
    PasswordRootSgn, PasswordRootRpt,
    QuestionConfig, Quit, UnknownError, 
}

// Types of boxes
pub enum BoxTypeMenu {
    Default, Main
}

pub trait HandlerDialog {
    fn get_box_default() -> Dialog; 
}

pub trait HandlerSdialog {
    fn set_box_default() -> Dialog; 
}

pub trait HandlerBox {
    fn get_text(&self) -> String; 
    fn handle(&mut self) -> Page; 
}

pub trait HandlerPage {
    fn next(&self) -> Page; 
    fn previous(&self) -> Page; 
}

pub trait HandlerCommand {
    fn do_command(&self); 
}



pub struct BoxMenu {}

impl HandlerDialog for BoxMenu {
    fn get_box_default() -> Dialog {
        Dialog::new() 
        .set_cancellabel(LABEL_BACK)
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_MENU)
        .set_height(HEIGHT_BOX_MENU)
    }
}

impl BoxMenu {
    pub fn choice(box_type:BoxTypeMenu, text: String, list: Vec<[String; 2]>) -> (Choice, Option<String>) {
        let dbox = match box_type {
            BoxTypeMenu::Main => Self::get_box_main(),
            _ => Self::get_box_default(),
        };
        Menu::new(text,HEIGHT_LIST_MENU, list).show_with(dbox).expect(EXP_DBOX)
    }

    pub fn get_page_from_selection_menu(list: &[(&str, Page)], selection: &str) -> Page {
        match list.into_iter().find(|(x,_)| x == &selection) {
            Some((_,y)) => {
                y.clone()
            },
            _ => Page::NoBoxFound,
        }
    }
    
    pub fn convert_page_list_to_dbox_list(list: &[(&str, Page)]) -> Vec<[String; 2]> {
        list.iter().map(|(x,_)| [x.to_string(), String::new()]).collect()
    }

    pub fn convert_string_list_to_dbox_list(list: &[[&str; 2]]) -> Vec<[String; 2]> {
        list.iter().map(|[x,y]| [x.to_string(), y.to_string()]).collect()
    }


    fn get_box_main() -> Dialog {
        Dialog::new()
        .set_cancellabel(LABEL_QUIT)
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_MENU)
        .set_height(HEIGHT_BOX_MENU)
    }
}

pub struct BoxInput {}

impl HandlerDialog for BoxInput {
    fn get_box_default() -> Dialog {
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_INPUT)
        .set_height(HEIGHT_BOX_INPUT)
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

impl HandlerDialog for BoxPassword {
    fn get_box_default() -> Dialog {
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_PASSWORD)
        .set_height(HEIGHT_BOX_PASSWORD)
        .set_insecure(true)
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

impl HandlerDialog for BoxQuestion {
    fn get_box_default() -> Dialog {
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_QUESTION)
        .set_height(HEIGHT_BOX_QUESTION)
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

pub struct BoxGauge {}

impl HandlerDialog for BoxGauge {
    fn get_box_default() -> Dialog {
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_MIXED_GAUGE)
        .set_height(HEIGHT_BOX_MIXED_GAUGE)
    }
}

impl BoxGauge {
    pub fn show(text: &str, percent: u8) -> () {
        Gauge::new(text, percent)
            .show_with(Self::get_box_default())
            .expect(EXP_DBOX)
    }
}

pub struct BoxMessage {
    dbox: Dialog,
    page: Page,
    text: String,
}

impl BoxMessage {
    pub fn handle(&self) -> Page {
        Message::new(&self.text)
            .show_with(&self.dbox)
            .expect(EXP_DBOX);
        self.page.clone()
    }

    pub fn new(dialog: Dialog, text: &str, page: Page) -> Self {
        BoxMessage {
            dbox: dialog,
            page: page,
            text: text.to_string()
        }
    }
}




/*
pub struct BoxForm {}

impl HandlerDialog for BoxForm {
    fn get_box_default() -> Dialog {
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(WIDTH_BOX_MIXED_GAUGE)
        .set_height(HEIGHT_BOX_MIXED_GAUGE)
    }
}

impl BoxForm {
    pub fn choice(text: String, list: Vec<(String, u8, u8, String, u8, u8, u8, u8)>) 
    -> (Choice, Option<String>)  {
        Form::new(text, HEIGHT_LIST_FORM, list)
            .show_with(Self::get_box_default())
            .expect(EXP_DBOX)
    }
}
*/
