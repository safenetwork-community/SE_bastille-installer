pub mod command;
mod list;
mod text;
pub mod ebox;
pub mod r#type;

use dialog::{backends::Dialog};

use std::path::{Path, PathBuf};

use dialog::Choice;
use regex::RegexSet;

use crate::app::dbox::command::*;
use crate::app::dbox::list::*;
use crate::app::dbox::text::*;
use crate::app::dbox::r#type::*;

use crate::app::constants::{TITLE, TITRFOQ};

pub struct BoxMenuMain<'a> {
    pub error_msg: &'a mut String,
}

impl HandlerBox for BoxMenuMain<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Main, self.get_text(), 
            BoxMenu::convert_page_list_to_dbox_list(&LIST_MENU_MAIN)) {
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

pub struct BoxMenuDevice<'a> {
    pub device: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuDevice<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(),  
            BoxMenu::convert_string_list_to_dbox_list(&LIST_MENU_DEVICE)) {
            (Choice::Yes, Some(device)) => {
                *self.device = device;
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

impl HandlerPage for BoxMenuDevice<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::MenuOperatingSystem,
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

pub struct BoxMenuOperatingSystem<'a> {
    pub os: &'a mut String,
    pub single_edit: bool,
}


impl HandlerBox for BoxMenuOperatingSystem<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_string_list_to_dbox_list(&LIST_MENU_OS)) {
            (Choice::Yes, Some(os)) => {
                *self.os = os;
                self.next()
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NoBoxFound,
        }
    }
     
    fn get_text(&self) -> String {
        TextMenuOperatingSystem {}.to_string()
    }
}

impl HandlerPage for BoxMenuOperatingSystem<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::MenuDrive,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuDevice,
            true => Page::MenuConfig,
        }
    }
}



pub struct BoxMenuDrive<'a> {
    pub drive: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuDrive<'_> {

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

impl HandlerPage for BoxMenuDrive<'_> {

    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::InputUsername,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuOperatingSystem,
            true => Page::MenuConfig,
        }
    }
}



pub struct BoxInputUsername<'a> {
    pub username: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxInputUsername<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
            .set_cancellabel("Back");
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

impl HandlerPage for BoxInputUsername<'_> {
    
    fn next(&self) -> Page {
        match self.single_edit {
            false => Page::InputUsergroups,
            true => Page::MenuConfig,
        }
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::MenuDrive,
            true => Page::MenuConfig,
        }
    }
}  

pub struct BoxInputUsergroups<'a> {
    pub username: &'a str,
    pub usergroups: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxInputUsergroups<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
        .set_cancellabel("Back");
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

impl HandlerPage for BoxInputUsergroups<'_> {

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

impl HandlerBox for BoxInputFullname<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
            .set_cancellabel("Back");
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

impl HandlerPage for BoxInputFullname<'_> {

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

impl HandlerBox for BoxPasswordUserSgn<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxPassword::get_box_default()
            .set_cancellabel("Back");
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

impl HandlerPage for BoxPasswordUserSgn<'_> {

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

impl HandlerBox for BoxPasswordUserRpt<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxPassword::get_box_default()
            .set_cancellabel("Back");
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

impl HandlerPage for BoxPasswordUserRpt<'_> {

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

impl HandlerBox for BoxPasswordRootSgn<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxPassword::get_box_default()
            .set_cancellabel("Back");
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

impl HandlerPage for BoxPasswordRootSgn<'_> {

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

impl HandlerBox for BoxPasswordRootRpt<'_>  {
    fn handle(&mut self) -> Page {
        let dbox = BoxPassword::get_box_default()
        .set_cancellabel("Back");
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

impl HandlerPage for BoxPasswordRootRpt<'_> {

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

impl HandlerBox for BoxMenuTimezoneRegion<'_> {
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

impl HandlerPage for BoxMenuTimezoneRegion<'_> {

    fn next(&self) -> Page {
        Page::MenuTimezoneZone
    }

    fn previous(&self) -> Page {
        match self.single_edit {
            false => Page::PasswordRootSgn,
            true => Page::MenuConfig,
        }
    }
}

pub struct BoxMenuTimezoneZone<'a>  {
    pub zone: &'a mut String,
    pub path: &'a Path,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuTimezoneZone<'_>  {
    fn handle(&mut self) -> Page {
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

impl HandlerPage for BoxMenuTimezoneZone<'_> {

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

impl HandlerBox for BoxMenuKeymapGuest<'_> {
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

impl HandlerPage for BoxMenuKeymapGuest<'_> {

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

impl HandlerBox for BoxMenuKeyvarGuest<'_>  {
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

impl HandlerPage for BoxMenuKeyvarGuest<'_> {

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

impl HandlerBox for BoxInputHostname<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
        .set_cancellabel("Back");
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

impl HandlerPage for BoxInputHostname<'_> {

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

impl HandlerBox for BoxQuestionConfig<'_> {

    fn handle(&mut self) -> Page {
        match BoxQuestion::choice(self.get_text(), None) {
            Choice::Yes => Page::GaugeInstallation,
            Choice::No => Page::MenuConfig,
            Choice::Escape => Page::Escape,
            _ => Page::Finish,
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

pub struct BoxGaugeInstallation<'a> {
    pub username: &'a str,
    pub fullname: &'a str,
    pub usergroups: &'a str,
    pub drive: &'a str,
    pub timezone_region: &'a str,
    pub timezone_zone: &'a str,
    pub keymap: &'a str,
    pub keyvar: &'a str,
    pub hostname: &'a str,
    pub list_installation: &'a [(&'a str, &'a mut dyn Fn())], 
}

impl HandlerBox for BoxGaugeInstallation<'_> {
    fn handle(&mut self) -> Page {
        
        
        /*  for c_done in 0..c_total {
            let percent = c_done / c_total;
            Gauge::new("test", percent)
            .show_with(Dialog::new()).expect("test");
            self.do_command();
        }*/

        let c_total: u8 = self.list_installation.len() as u8;

        for c_done in 0..c_total {
            let percent: u8 = c_done as u8 * 100 / c_total;
            let tnc = &mut &self.list_installation[c_done as usize];

            BoxGauge::show(tnc.0, percent);
            tnc.1();
        }

        Page::MenuMain
    }

    fn get_text(&self) -> String {
        String::new()
        /*TextGaugeInstallation {
            functions: &self.functions,
        }.to_string()*/
    }
}

impl HandlerCommand for BoxGaugeInstallation<'_> {
    fn do_command(&self) {
        CommandInstall::show_elapsed_time();
    }
}

pub struct BoxMenuConfig<'a> {
    pub single_edit: &'a mut bool,
}

impl HandlerBox for BoxMenuConfig<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_page_list_to_dbox_list(&LIST_MENU_CONFIG)) {
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

impl HandlerBox for BoxMenuKeymapHost<'_> {
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

impl HandlerBox for BoxMenuKeyvarHost<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(keyvar)) => {
                *self.keyvar = keyvar.clone();
                CommandInstall::setup_keymap(self.keymap, &keyvar);
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

pub struct DBoxPool {
    pub message_finish: BoxMessage
}

impl DBoxPool {
    pub fn new() -> Self {
        DBoxPool {
            message_finish: BoxMessage::new(
                Dialog::new()
                .set_backtitle(TITRFOQ)
                .set_title(TITLE)
                .set_oklabel(LABEL_QUIT),
                "Installation Finished!\nPress Quit to terminate program.",
                Page::Quit),
        }    
    }
}
