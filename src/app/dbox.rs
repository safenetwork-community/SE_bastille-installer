mod command;
mod list;
mod text;
pub mod r#type;

use std::path::{Path, PathBuf};

use dialog::Choice;
use regex::RegexSet;

use crate::app::r#box::Page;
use crate::app::dbox::command::*;
use crate::app::dbox::list::*;
use crate::app::dbox::text::*;
use crate::app::dbox::r#type::*;
use crate::app::dbox::r#type::PageHandler;



pub struct BoxMenuMain  {
    pub error_msg: String,
}

impl PageHandler for BoxMenuMain {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Main, self.get_text(), 
            BoxMenu::convert_to_dbox_list(&LIST_MENU_MAIN)) {
            (Choice::Yes, Some(choice)) => {
                BoxMenu::get_page_from_selection_menu(&LIST_MENU_MAIN, &choice)
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

    fn get_text(&self) -> String {
        TextMenuMain {}.to_string()
    }
}

pub struct BoxInputUsername {
    pub username: String,
}

impl PageHandler for BoxInputUsername {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), 
        DEFAULT_USERNAME, Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextInputUsername {}.to_string()
    }
}


pub struct BoxInputUsergroups<'a> {
    pub username: &'a str,
    pub usergroups: String,
}

impl PageHandler for BoxInputUsergroups<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), EMPTY, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                self.usergroups = input_text;
                Page::Fullname
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Username,
            _ => Page::NoBoxFound,
        }
    }
    fn get_text(&self) -> String {
        TextInputUsergroups { 
            username: &*self.username,
        }.to_string()
    }
}


pub struct BoxInputFullname<'a> {
    pub username: &'a str,
    pub fullname: String,
}

impl PageHandler for BoxInputFullname<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), DEFAULT_FULLNAME, Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextInputFullname {
            username: &*self.username,
        }.to_string()
    }
}

pub struct BoxPasswordUserSgn<'a> {
    pub username: &'a str,
    pub password_user: String,
}

impl PageHandler for BoxPasswordUserSgn<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextPasswordUserSgn {
            username: &*self.username,
        }.to_string()
    }
}

pub struct BoxPasswordUserRpt<'a> {
    pub username: &'a str,
    pub password_user: &'a str,
}

impl PageHandler for BoxPasswordUserRpt<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextPasswordUserRpt {
            username: &*self.username,
        }.to_string()
    }
}

pub struct BoxPasswordRootSgn {
    pub password_root: String,
}

impl PageHandler for BoxPasswordRootSgn {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextPasswordRootSgn {}.to_string()
    }
}

pub struct BoxPasswordRootRpt<'a> {
    pub password_root: &'a str,
}

impl PageHandler for BoxPasswordRootRpt<'_>  {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if self.password_root.eq(&password) {
                    Page::Drive
                }
                else { Page::NoMatchPasswordRoot }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::PasswordRootSgn,
            _ => Page::NoBoxFound,
        }
    }
    fn get_text(&self) -> String {
        TextPasswordRootRpt {}.to_string()
    }
}

pub struct BoxMenuDrive {
    pub drive: String,
}

impl PageHandler for BoxMenuDrive {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::drives()) {
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
    
    fn get_text(&self) -> String {
        TextMenuDrive {}.to_string()
    }
}

pub struct BoxMenuTimezoneRegion {
    pub region: String,
    pub pathbuf: PathBuf,
}

impl PageHandler for BoxMenuTimezoneRegion {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::timeregion()) {
            (Choice::Yes, Some(region)) => {
                self.region = region.clone();
                self.pathbuf = Path::new(PATH_ZONEINFO).join(region);
                Page::TimezoneZone
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Drive,
            _ => Page::NoBoxFound,
        }

    }
    
    fn get_text(&self) -> String {
        TextMenuTimezoneRegion {}.to_string()
    }
}

pub struct BoxMenuTimezoneZone<'a>  {
    pub zone: String,
    pub path: &'a Path,
}

impl PageHandler for BoxMenuTimezoneZone<'_>  {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxMenu::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::timezone(self.path)) {
            (Choice::Yes, Some(zone)) => {
                self.zone = zone;
                Page::KeymapGuest
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::Drive,
            _ => Page::NoBoxFound,
        }
    }
    
    fn get_text(&self) -> String {
        TextMenuTimezoneZone {}.to_string()
    }
}

pub struct BoxMenuKeymapGuest {
    pub keymap: String,
    pub pathbuf: PathBuf,
}

impl PageHandler for BoxMenuKeymapGuest {
 fn handle(&mut self) -> Page {
    match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap = keymap.clone();
                self.pathbuf = Path::new(PATH_BKEYMAP).join(keymap);
                Page::KeyvarGuest
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::TimezoneRegion,
            _ => Page::NoBoxFound,
        }

    }
    fn get_text(&self) -> String {
        TextMenuKeymapGuest {}.to_string()
    }
}

pub struct BoxMenuKeyvarGuest<'a>  {
    pub keyvar: String,
    pub path: &'a Path,
}

impl PageHandler for BoxMenuKeyvarGuest<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar = keyvar;
                Page::Hostname
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapGuest,
            _ => Page::NoBoxFound,
        }

    }
    fn get_text(&self) -> String {
        TextMenuKeyvarGuest {}.to_string()
    }
}

pub struct BoxInputHostname {
    pub hostname: String,
}

impl PageHandler for BoxInputHostname {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), 
        DEFAULT_HOSTNAME, Some(dbox)) {
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
    fn get_text(&self) -> String {
        TextInputHostname {}.to_string()
    }
}

pub struct BoxQuestionConfirmation<'a> {
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

impl PageHandler for BoxQuestionConfirmation<'_> {

    fn handle(&mut self) -> Page {
        match BoxQuestion::choice(self.get_text(), None) {
            Choice::Yes => Page::Finish,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::NoBoxFound,
        }
    }

    fn get_text(&self) -> String {
        TextQuestionConfirmation {
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
}

pub struct BoxMenuConfig {}

impl PageHandler for BoxMenuConfig  {
    fn handle(&mut self) -> Page {
    match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_to_dbox_list(&LIST_MENU_CONFIG)) {
            (Choice::Yes, Some(choice_menu)) => {
                BoxMenu::get_page_from_selection_menu(&LIST_MENU_CONFIG, &choice_menu) 
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapGuest,
            _ => Page::NoBoxFound,
        }
    }

    fn get_text(&self) -> String {
        TextMenuConfig {}.to_string()
    }
}

pub struct BoxMenuKeymapHost {
    pub keymap: String,
    pub pathbuf: PathBuf,
}

impl PageHandler for BoxMenuKeymapHost {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(keymap)) => {
                self.keymap = keymap.clone();
                self.pathbuf = Path::new(PATH_BKEYMAP).join(keymap);
                Page::KeyvarHost
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::MenuMain,
            _ => Page::NoBoxFound,
        }
    }
    fn get_text(&self) -> String {
        TextMenuKeymapHost {}.to_string()
    }
}

pub struct BoxMenuKeyvarHost<'a>  {
    pub keymap: &'a str,
    pub keyvar: String,
    pub path: &'a Path,
}

impl PageHandler for BoxMenuKeyvarHost<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(keyvar)) => {
                self.keyvar = keyvar.clone();
                Command::setup_keymap(self.keymap, &keyvar);
                Page::Username
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::KeymapHost,
            _ => Page::NoBoxFound,
        }

    }
    fn get_text(&self) -> String {
        TextMenuKeyvarHost {}.to_string()
    }
}
