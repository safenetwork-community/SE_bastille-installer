mod text;
mod r#type;

use dialog::Choice;

use crate::app::r#box::Page;
use crate::app::dbox::text::*;
use crate::app::dbox::r#type::*;

pub trait QuestionBoxHandler {
    fn get_text(&self) -> String; 
    fn handle(&mut self) -> Page; 
}

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

impl QuestionBoxHandler for ConfirmationBox<'_> {
    fn get_text(&self) -> String {
        ConfirmationText {
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

    fn handle(&mut self) -> Page {
        match QuestionBox::choice(self.get_text(), None) {
            Choice::Yes => Page::Finish,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::NoBoxFound,
        }
    }
}
