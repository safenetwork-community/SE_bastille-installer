mod text;
pub mod ebox;
pub mod r#type;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

use dialog::{backends::Dialog, Choice};
use regex::RegexSet;


use crate::app::commands::execute::CommandExecute;
use crate::app::commands::list::*;
use crate::app::dbox::text::*;
use crate::app::dbox::r#type::*;

use crate::shared::constants::dbox::*;
use crate::shared::constants::string::EMPTY;

pub struct BoxMenuMain<'a> {
    pub msg_error: &'a mut String,
}

impl HandlerBox for BoxMenuMain<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Main, self.get_text(), 
            BoxMenu::convert_page_list_to_dbox_list(LIST_MENU_MAIN)) {
            (Choice::Yes, Some(choice)) => {
                BoxMenu::get_page_from_selection_menu(LIST_MENU_MAIN, &choice)
            },
            (Choice::Escape, Some(msg_txt)) => {
                match msg_txt.as_str() {
                    EMPTY => Page::Escape,
                    ERROR_EMPTY_MENU => Page::EmptyMenu,
                    _ => {
                        *self.msg_error = msg_txt;
                        Page::ErrorUnknown
                    },
                }
            },
            (Choice::Cancel, _) => Page::Quit,
            _ => Page::NotFoundBox, 
        }
    }

    fn get_text(&self) -> String {
        TextMenuMain {}.to_string()
    }
}

pub struct BoxMenuDevice<'a> {
    pub name_device: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuDevice<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(),  
            BoxMenu::convert_string_list_to_dbox_list(LIST_MENU_DEVICE)) {
            (Choice::Yes, Some(device)) => {
                *self.name_device = device;
                self.next()
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub name_os: &'a mut String,
    pub single_edit: bool,
}


impl HandlerBox for BoxMenuOperatingSystem<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_string_list_to_dbox_list(LIST_MENU_OS)) {
            (Choice::Yes, Some(os)) => {
                *self.name_os = os;
                self.next()
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub name_drive: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuDrive<'_> {

    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::drives()) {
            (Choice::Yes, Some(drive)) => {
                *self.name_drive = drive;
                self.next()
            },
            (Choice::Yes, None) => Page::EmptyMenu,
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub name_user: &'a mut String,
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
                        *self.name_user = input_text;
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
            _ => Page::NotFoundBox,
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
    pub name_user: &'a str,
    pub groups_user: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxInputUsergroups<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
        .set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), EMPTY, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                *self.groups_user = input_text;
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
        }
    }
 
    fn get_text(&self) -> String {
        TextInputUsergroups { 
            username: self.name_user,
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
    pub name_user: &'a str,
    pub name_full: &'a mut String,
    pub single_edit: bool,
}

impl HandlerBox for BoxInputFullname<'_> {
    fn handle(&mut self) -> Page {
        let dbox = BoxInput::get_box_default()
            .set_cancellabel("Back");
        match BoxInput::choice(self.get_text(), DEFAULT_FULLNAME, Some(dbox)) {
            (Choice::Yes, Some(input_text)) => {
                if !input_text.is_empty() {
                    *self.name_full = input_text;
                    self.next()
                }
                else { Page::EmptyFullname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
        }
    }
 
    fn get_text(&self) -> String {
        TextInputFullname {
            username: self.name_user,
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
    pub name_user: &'a str,
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
            _ => Page::NotFoundBox,
        }

    }
    
    fn get_text(&self) -> String {
        TextPasswordUserSgn {
            username: self.name_user,
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
    pub name_user: &'a str,
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
            _ => Page::NotFoundBox,
        }
    }
  
    fn get_text(&self) -> String {
        TextPasswordUserRpt {
            username: self.name_user,
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
            _ => Page::NotFoundBox,
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
            _ => Page::NotFoundBox,
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
    pub path: &'a mut PathBuf,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuTimezoneRegion<'_> {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::timeregion()) {
            (Choice::Yes, Some(region)) => {
                *self.region = region.clone();
                *self.path = Path::new(PATH_ZONEINFO).join(region);
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
            _ => Page::NotFoundBox,
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
    pub map_key: &'a mut String,
    pub path: &'a mut PathBuf,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuKeymapGuest<'_> {
 fn handle(&mut self) -> Page {
    match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(map_key)) => {
                *self.map_key = map_key.clone();
                *self.path = Path::new(PATH_BKEYMAP).join(map_key);
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub var_key: &'a mut String,
    pub path: &'a Path,
    pub single_edit: bool,
}

impl HandlerBox for BoxMenuKeyvarGuest<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(var_key)) => {
                *self.var_key = var_key;
                self.next()
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub name_host: &'a mut String,
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
                        *self.name_host = input_text;
                        self.next()
                    }
                    else { Page::InvalidHostname }
                }
                else { Page::EmptyHostname }
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => self.previous(),
            _ => Page::NotFoundBox,
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
    pub groups_user: &'a str,
    pub map_key: &'a str,
    pub name_drive: &'a str,
    pub name_full: &'a str,
    pub name_host: &'a str,
    pub name_user: &'a str,
    pub region_timezone: &'a str,
    pub var_key: &'a str,
    pub zone_timezone: &'a str,
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
            usergroups: self.groups_user,
            drive: self.name_drive, 
            fullname: self.name_full,
            hostname: self.name_host,
            username: self.name_user,
            timezone_region: self.region_timezone,
            timezone_zone: self.zone_timezone,
            keymap: self.map_key,
            keyvar: self.var_key,
        }.to_string()
    }
}

pub struct BoxGaugeInstallation<'a> {
    pub list_command: Vec<(String, Option<Command>)>, 
    pub msg_error: &'a mut String,
}

impl HandlerGauge for BoxGaugeInstallation<'_> {
    fn handle(&mut self) -> Page {
        let c_total = self.list_command.len();

        for c_done in 0..c_total {
            let percent: u8 = (c_done * 100 / c_total) as u8;
            let tuple_command = &mut self.list_command[c_done];
            let option_command = &mut tuple_command.1; 

            BoxGauge::show(tuple_command.0.as_str(), percent);
            sleep(Duration::from_millis(1000));
            match option_command {
                Some(command) => { 
                    let mut display_command :OsString = OsString::from(command.get_program());
                        display_command.push(" ");
                        display_command.push(command.get_args()
                        .collect::<Vec<_>>()
                        .join(OsStr::new(" ")));  
                    let result_command = command
                        .output()
                        .unwrap_or_else(|_| panic!("Failed to execute process:\n\n{:?}", display_command));
                    info!("display_command:{:?}", display_command.clone().into_string());
                    info!("result_command:{:?}", result_command);

                    match result_command.status.success() {
                        false => {
                           *self.msg_error = format!("Process returned an error:\n\n{:?}\n\nOutput stderr:\n\n{}",
                                display_command,  
                                String::from_utf8(result_command.stderr).map_err(|non_utf8| 
                                String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()).unwrap()); 
                            return Page::FailedCommand;
                        }
                        true => continue,
                    }
                },
                None => {},
            }    
        }

        Page::Finish
    }
}

pub struct BoxMenuConfig<'a> {
    pub single_edit: &'a mut bool,
}

impl HandlerBox for BoxMenuConfig<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            BoxMenu::convert_page_list_to_dbox_list(LIST_MENU_CONFIG)) {
            (Choice::Yes, Some(choice_menu)) => {
                *self.single_edit = true;
                BoxMenu::get_page_from_selection_menu(LIST_MENU_CONFIG, &choice_menu) 
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => {
                *self.single_edit = false;
                Page::QuestionConfig
            },
            _ => Page::NotFoundBox,
        }
    }
 
    fn get_text(&self) -> String {
        TextMenuConfig {}.to_string()
    }
}

pub struct BoxMenuKeymapHost<'a> {
    pub map_key: &'a mut String,
    pub path: &'a mut PathBuf,
}

impl HandlerBox for BoxMenuKeymapHost<'_> {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keymap()) {
            (Choice::Yes, Some(keymap)) => {
                *self.map_key = keymap.clone();
                *self.path = Path::new(PATH_BKEYMAP).join(keymap);
                Page::MenuKeyvarHost
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::MenuMain,
            _ => Page::NotFoundBox,
        }
    }
 
    fn get_text(&self) -> String {
        TextMenuKeymapHost {}.to_string()
    }
}

pub struct BoxMenuKeyvarHost<'a>  {
    pub map_key: &'a str,
    pub path: &'a Path,
    pub var_key: &'a mut String,
}

impl HandlerBox for BoxMenuKeyvarHost<'_>  {
    fn handle(&mut self) -> Page {
        match BoxMenu::choice(BoxTypeMenu::Default, self.get_text(), 
            ListFromCommand::keyvars(self.path)) {
            (Choice::Yes, Some(var_key)) => {
                *self.var_key = var_key.clone();
                CommandExecute::setup_keymap(self.map_key, &var_key);
                Page::MenuMain
            },
            (Choice::Escape, _) => Page::Escape,
            (Choice::Cancel, _) => Page::MenuKeymapHost,
            _ => Page::NotFoundBox,
        }

    }
  
   fn get_text(&self) -> String {
        TextMenuKeyvarHost {}.to_string()
    }
}

pub fn get_box_message_finish() -> BoxMessage {
    BoxMessage::new(
        Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_title(TITLE)
        .set_width(45)
        .set_height(6),
        "Installation Finished!\nPress Enter to go back to the Main Menu.",
        Page::MenuMain)
}
