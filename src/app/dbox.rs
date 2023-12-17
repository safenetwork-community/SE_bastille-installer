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
use crate::app::dbox::r#type::BoxHandler;

pub struct BoxMenuMain<'a> {
    pub error_msg: &'a mut String,
}

impl BoxHandler for BoxMenuMain<'_> {

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
                        *self.error_msg = msg_txt;
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

pub struct BoxInputUsername<'a> {
    pub username: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxInputUsername<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), 
        DEFAULT_USERNAME, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_USERNAME).unwrap().is_match(&input_text) {
                        *self.username = input_text;
                        self.next()                 
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
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }

        fn get_text(&self) -> String {
        TextInputUsername {}.to_string()
    }
}

impl PageHandler for BoxInputUsername<'_> {
    
    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::InputUsergroups,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuMain,
            true => Page::MenuConfig,
        }
    }
}  

pub struct BoxInputUsergroups<'a> {
    pub username: &'a str,
    pub usergroups: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxInputUsergroups<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), EMPTY, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                *self.usergroups = input_text;
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
 
    fn get_text(&self) -> String {
        TextInputUsergroups { 
            username: &*self.username,
        }.to_string()
    }
}

impl PageHandler for BoxInputUsergroups<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::InputFullname,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::InputUsername,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxInputFullname<'a> {
    pub username: &'a str,
    pub fullname: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxInputFullname<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), DEFAULT_FULLNAME, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    *self.fullname = input_text;
                    self.next()
                }
                else { Page::EmptyFullname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
 
    fn get_text(&self) -> String {
        TextInputFullname {
            username: &*self.username,
        }.to_string()
    }
}

impl PageHandler for BoxInputFullname<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordUserSgn,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::InputUsergroups,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxPasswordUserSgn<'a> {
    pub username: &'a str,
    pub password_user: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxPasswordUserSgn<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    *self.password_user = password;
                    self.next()
                }
                else { Page::EmptyPasswordUser }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }

    }
    
    fn get_text(&self) -> String {
        TextPasswordUserSgn {
            username: &*self.username,
        }.to_string()
    }
}

impl PageHandler for BoxPasswordUserSgn<'_> {

    fn next(&self) -> Page {
        Page::PasswordUserRpt
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::InputFullname,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxPasswordUserRpt<'a> {
    pub username: &'a str,
    pub password_user: &'a str,
    pub single_edit: bool,
}

impl BoxHandler for BoxPasswordUserRpt<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if self.password_user.eq(&password) {
                    self.next()
                }
                else { Page::NoMatchPasswordUser }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
  
    fn get_text(&self) -> String {
        TextPasswordUserRpt {
            username: &*self.username,
        }.to_string()
    }
}

impl PageHandler for BoxPasswordUserRpt<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordRootSgn,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordUserSgn,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxPasswordRootSgn<'a> {
    pub password_root: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxPasswordRootSgn<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if !password.is_empty() {
                    *self.password_root = password;
                    self.next()
                }
                else { Page::EmptyPasswordRoot }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
  
    fn get_text(&self) -> String {
        TextPasswordRootSgn {}.to_string()
    }
}

impl PageHandler for BoxPasswordRootSgn<'_> {

    fn next(&self) -> Page {
        Page::PasswordRootRpt
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordRootSgn,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxPasswordRootRpt<'a> {
    pub password_root: &'a str,
    pub single_edit: bool,
}

impl BoxHandler for BoxPasswordRootRpt<'_>  {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxPassword::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxPassword::choice(self.get_text(), Some(dbox)) {
            (Choice::Yes, Some(password)) => {
                if self.password_root.eq(&password) {
                    self.next()
                }
                else { Page::NoMatchPasswordRoot }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
  
    fn get_text(&self) -> String {
        TextPasswordRootRpt {}.to_string()
    }
}

impl PageHandler for BoxPasswordRootRpt<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::MenuDrive,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordRootSgn,
            true => Page::MenuConfig,
        }
    }
}


pub struct BoxMenuDrive<'a> {
    pub drive: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxMenuDrive<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::drives()) {
            (Choice::Yes, Some(drive)) => {
                *self.drive = drive;
                self.next()
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
     
    fn get_text(&self) -> String {
        TextMenuDrive {}.to_string()
    }
}

impl PageHandler for BoxMenuDrive<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::MenuTimezoneRegion,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordRootSgn,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxMenuTimezoneRegion<'a> {
    pub region: &'a mut String,
    pub pathbuf: &'a mut PathBuf,
    pub single_edit: bool,
}

impl BoxHandler for BoxMenuTimezoneRegion<'_> {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::timeregion()) {
            (Choice::Yes, Some(region)) => {
                *self.region = region.clone();
                *self.pathbuf = Path::new(PATH_ZONEINFO).join(region);
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }

    }
     
    fn get_text(&self) -> String {
        TextMenuTimezoneRegion {}.to_string()
    }
}

impl PageHandler for BoxMenuTimezoneRegion<'_> {

    fn next(&self) -> Page {
        Page::MenuTimezoneZone
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuDrive,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxMenuTimezoneZone<'a>  {
    pub zone: &'a mut String,
    pub path: &'a Path,
    pub single_edit: bool,
}

impl BoxHandler for BoxMenuTimezoneZone<'_>  {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxMenu::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::timezone(self.path)) {
            (Choice::Yes, Some(zone)) => {
                *self.zone = zone;
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
     
    fn get_text(&self) -> String {
        TextMenuTimezoneZone {}.to_string()
    }
}

impl PageHandler for BoxMenuTimezoneZone<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::MenuKeymapGuest,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuTimezoneRegion,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxMenuKeymapGuest<'a> {
    pub keymap: &'a mut String,
    pub pathbuf: &'a mut PathBuf,
    pub single_edit: bool,
}

impl BoxHandler for BoxMenuKeymapGuest<'_> {
 fn handle(&mut self) -> Page {
    match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(keymap)) => {
                *self.keymap = keymap.clone();
                *self.pathbuf = Path::new(PATH_BKEYMAP).join(keymap);
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }

    }
   
    fn get_text(&self) -> String {
        TextMenuKeymapGuest {}.to_string()
    }
}

impl PageHandler for BoxMenuKeymapGuest<'_> {

    fn next(&self) -> Page {
        Page::MenuKeyvarGuest
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuTimezoneRegion,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxMenuKeyvarGuest<'a>  {
    pub keyvar: &'a mut String,
    pub path: &'a Path,
    pub single_edit: bool,
}

impl BoxHandler for BoxMenuKeyvarGuest<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(keyvar)) => {
                *self.keyvar = keyvar;
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }

    }
   
    fn get_text(&self) -> String {
        TextMenuKeyvarGuest {}.to_string()
    }
}

impl PageHandler for BoxMenuKeyvarGuest<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::InputHostname,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuKeymapGuest,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxInputHostname<'a> {
    pub hostname: &'a mut String,
    pub single_edit: bool,
}

impl BoxHandler for BoxInputHostname<'_> {
    fn handle(&mut self) -> Page {
        let mut dbox = BoxInput::get_box_default();
        dbox.set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), 
        DEFAULT_HOSTNAME, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    if !RegexSet::new(REGEX_HOSTNAME).unwrap().is_match(&input_text) {
                        *self.hostname = input_text;
                        self.next()
                    }
                    else { Page::InvalidHostname }
                }
                else { Page::EmptyHostname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }

    }
   
    fn get_text(&self) -> String {
        TextInputHostname {}.to_string()
    }
}

impl PageHandler for BoxInputHostname<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::QuestionConfig,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuKeymapGuest,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxQuestionConfig<'a> {
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

impl BoxHandler for BoxQuestionConfig<'_> {

    fn handle(&mut self) -> Page {
        match BoxQuestion::choice(self.get_text(), None) {
            Choice::Yes => Page::Finish,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::NoBoxFound,
        }
    }
 
    fn get_text(&self) -> String {
        TextQuestionConfig {
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

pub struct BoxMenuConfig<'a> {
    pub single_edit: &'a mut bool,
}

impl BoxHandler for BoxMenuConfig<'_>  {
    fn handle(&mut self) -> Page {
    match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_to_dbox_list(&LIST_MENU_CONFIG)) {
            (Choice::Yes, Some(choice_menu)) => {
                *self.single_edit = true;
                BoxMenu::get_page_from_selection_menu(&LIST_MENU_CONFIG, &choice_menu) 
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => {
                *self.single_edit = false;
                Page::QuestionConfig
            },
            _ => Page::NoBoxFound,
        }
    }
 
    fn get_text(&self) -> String {
        TextMenuConfig {}.to_string()
    }
}

pub struct BoxMenuKeymapHost<'a> {
    pub keymap: &'a mut String,
    pub pathbuf: &'a mut PathBuf,
}

impl BoxHandler for BoxMenuKeymapHost<'_> {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(keymap)) => {
                *self.keymap = keymap.clone();
                *self.pathbuf = Path::new(PATH_BKEYMAP).join(keymap);
                Page::MenuKeyvarHost
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
    pub keyvar: &'a mut String,
    pub path: &'a Path,
}

impl BoxHandler for BoxMenuKeyvarHost<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(keyvar)) => {
                *self.keyvar = keyvar.clone();
                Command::setup_keymap(self.keymap, &keyvar);
                Page::MenuMain
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::MenuKeymapHost,
            _ => Page::NoBoxFound,
        }

    }
  
   fn get_text(&self) -> String {
        TextMenuKeyvarHost {}.to_string()
    }
}
