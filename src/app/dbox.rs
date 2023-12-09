use const_format::concatcp;
use dialog::{backends::Dialog, Choice, DialogBox, Question};

use crate::app::constants;
use crate::forms::*;

const TITRFOQ: &str = concatcp!("Èstalxr d sistêmbêstur d Bastij", " ", constants::VERSION);
const TITLE: &str = concatcp!(constants::OS, " ", constants::VERSION);

// dboxes, mboxes and exits
#[derive(Clone)]
pub enum Page {
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

const EXP_DBOX: &str = "Could not display dialog box.";

// Dimensions question box
const HEIGHT_BOX_QUESTION: u32 = 20;
const WIDTH_BOX_QUESTION: u32 = 90;



pub struct ConfirmationBox<'a> {
    pub username: &'a str,
    pub fullname: &'a str,
    pub usergroups: &'a str,
    pub drive: &'a str,
    pub timezone_region: &'a str,
    pub timezone_zone: &'a str,
    pub keymap: &'a str,
    pub keyvar: &'a str,
    pub hostname: &'a str,
}

impl ConfirmationBox<'_> {

    pub fn new<'a>() -> ConfirmationBox<'a> {
        ConfirmationBox {
            username: "",
            fullname: "",
            usergroups: "",
            drive: "",
            timezone_region: "",
            timezone_zone: "",
            keymap: "",
            keyvar: "",
            hostname: "",
        }
    }

    fn display_form(&self) -> String {
        ConfirmationForm {
            username: &*self.username,
            fullname: &*self.fullname,
            usergroups: &*self.usergroups,
            drive: &*self.drive, 
            timezone_region: &*self.timezone_region,
            timezone_zone: &*self.timezone_zone,
            keymap: &*self.keymap,
            keyvar: &*self.keyvar,
            hostname: &*self.hostname,
        }.to_string()
    }

    pub fn handle(&mut self) -> Page {
        match Self::question_box(self.display_form(), None) {
            Choice::Yes => Page::Finish,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::NoBoxFound,
        }
    }

    fn question_box(text: String, dbox: Option<Dialog>) -> Choice {
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
}


