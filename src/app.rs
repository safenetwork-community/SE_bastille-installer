mod dbox;
mod commands;
mod install;

use anyhow::{anyhow, Error, Result};
use std::path::{Path, PathBuf};

use crate::app::commands::execute::CommandExecute;
use crate::app::dbox::*;
use crate::app::dbox::ebox::*;
use crate::app::dbox::r#type::*;
use crate::app::install::ListCommand;

use crate::shared::constants::error::ErrorInstaller; 

// Exit texts
pub const MSG_EXIT_ESCAPE: &str =   "Escape pressed, exiting..";
pub const MSG_EXIT_QUIT: &str =     "Quit pressed, exiting..";

pub struct App {
    groups_user: String,
    key_pub: String,
    locale: String,
    map_key_guest: String,
    map_key_host: String, 
    msg_error: String,
    name_device: String,
    name_drive: String,
    name_host: String,
    name_user: String,
    name_full: String,
    password_user: String,
    password_root: String,
    path_timezone: PathBuf, 
    path_var_key: PathBuf,
    region_timezone: String,
    single_edit: bool,
    var_key_guest: String,
    var_key_host: String, 
    zone_timezone: String,
}

impl App {
    
    pub fn new() -> Self {
        App {
            groups_user: String::new(),
            key_pub: String::new(),
            locale: String::new(),
            map_key_guest: String::new(),
            map_key_host: String::new(),
            msg_error: String::new(),
            name_device: String::new(),
            name_drive: String::new(),
            name_full: String::new(),
            name_host: String::new(),
            name_user: String::new(),
            var_key_guest: String::new(),
            var_key_host: String::new(),
            path_timezone: PathBuf::new(),
            path_var_key: PathBuf::new(),
            password_user: String::new(),
            password_root: String::new(),
            region_timezone: String::new(),
            single_edit: false,
            zone_timezone: String::new(),
        }    
    }
    
    fn get_box_menu_main(&mut self) -> BoxMenuMain<'_> {
        BoxMenuMain {
            msg_error: &mut self.msg_error,
        }
    }


    fn get_box_input_name_user(&mut self) -> BoxInputUsername<'_> {
        BoxInputUsername {
            single_edit: self.single_edit,
            name_user: &mut self.name_user,
        }
    }

    fn get_box_input_groups_user(&mut self) -> BoxInputUsergroups<'_> {
        BoxInputUsergroups {
            single_edit: self.single_edit,
            name_user: &self.name_user,
            groups_user: &mut self.groups_user,
        }
    }

    fn get_box_input_name_full(&mut self) -> BoxInputFullname<'_> {
        BoxInputFullname {
            single_edit: self.single_edit,
            name_user: &self.name_user,
            name_full: &mut self.name_full,
        }
    }

    fn get_box_password_user_sign(&mut self) -> BoxPasswordUserSgn<'_> {
        BoxPasswordUserSgn {
            single_edit: self.single_edit,
            name_user: &self.name_user,
            password_user: &mut self.password_user,
        }
    }

    fn get_box_password_user_repeat(&self) -> BoxPasswordUserRpt<'_> {
        BoxPasswordUserRpt {
            single_edit: self.single_edit,
            name_user: &self.name_user,
            password_user: &self.password_user,
        }
    }

    fn get_box_password_root_sign(&mut self) -> BoxPasswordRootSgn {
        BoxPasswordRootSgn {
            single_edit: self.single_edit,
            password_root: &mut self.password_root,
        }
    }

    fn get_box_password_root_repeat(&self) -> BoxPasswordRootRpt {
        BoxPasswordRootRpt {
            single_edit: self.single_edit,
            password_root: &self.password_root,
        }
    }

    fn get_box_menu_name_device(&mut self) -> BoxMenuDevice {
        BoxMenuDevice {
            name_device: &mut self.name_device,
            single_edit: self.single_edit,
        }
    }

    fn get_box_menu_name_drive(&mut self) -> BoxMenuDrive {
        BoxMenuDrive {
            name_drive: &mut self.name_drive,
            single_edit: self.single_edit,
        }
    }

    fn get_box_menu_region_timezone(&mut self) -> BoxMenuTimezoneRegion {
        BoxMenuTimezoneRegion {
            single_edit: self.single_edit,
            region: &mut self.region_timezone,
            path: &mut self.path_timezone,
        }
    }

    fn get_box_menu_zone_timezone(&mut self) -> BoxMenuTimezoneZone<'_>  {
        BoxMenuTimezoneZone {
            single_edit: self.single_edit,
            zone: &mut self.zone_timezone,
            path: &self.path_timezone,
        }
    }

    fn get_box_menu_map_key_guest(&mut self) -> BoxMenuKeymapGuest {
        BoxMenuKeymapGuest {
            single_edit: self.single_edit,
            map_key: &mut self.map_key_guest,
            path: &mut self.path_var_key,
        }
    }

    fn get_box_menu_var_key_guest(&mut self) -> BoxMenuKeyvarGuest<'_>  {
        BoxMenuKeyvarGuest {
            single_edit: self.single_edit,
            var_key: &mut self.var_key_guest,
            path: &self.path_var_key,
        }
    }

    fn get_box_input_name_host(&mut self) -> BoxInputHostname {
        BoxInputHostname {
            single_edit: self.single_edit,
            name_host: &mut self.name_host, 
        }
    }
 
    fn get_box_question_config(&self) -> BoxQuestionConfig {
        BoxQuestionConfig {
            groups_user: &self.groups_user,
            map_key: &self.map_key_guest,
            name_drive: &self.name_drive, 
            name_full: &self.name_full,
            name_host: &self.name_host,
            name_user: &self.name_user,
            region_timezone: &self.region_timezone,
            var_key: &self.var_key_guest,
            zone_timezone: &self.zone_timezone,
        }
    }

    fn get_box_mixed_gauge_installation(&mut self) -> BoxGaugeInstallation {

        BoxGaugeInstallation {
            builder_list_command: self.get_builder_list_command(),
            msg_error: &mut self.msg_error,
        }
    }

    fn get_box_mixed_gauge_test_installation(&mut self) -> BoxGaugeInstallation {
        self.key_pub = String::from("id_pjehrsohmehj_folaht.pub");
        self.locale = String::from("be_FR.utf8");
        self.name_drive = String::from("sda");
        self.name_device = String::from("rpi4");
        self.name_full = String::from("Fôlat Pjêrsômêj");
        self.name_host = String::from("Rezosur-uq");
        self.name_user = String::from("folaht");
        self.password_root = String::from("mopahsrasin");
        self.password_user = String::from("mopahs");
        self.region_timezone = String::from("Europe");
        self.var_key_guest = String::from("yr-af");
        self.zone_timezone = String::from("Amsterdam");

        BoxGaugeInstallation {
            builder_list_command: self.get_builder_list_command(),
            msg_error: &mut self.msg_error,
        }
    }

    fn get_box_menu_config(&mut self) -> BoxMenuConfig {
        BoxMenuConfig {
            single_edit: &mut self.single_edit,
        }
    }

    fn get_box_menu_map_key_host(&mut self) -> BoxMenuKeymapHost<'_> {
        BoxMenuKeymapHost {
            map_key: &mut self.map_key_host,
            path: &mut self.path_var_key,
        }
    }

    fn get_box_menu_var_key_host(&mut self) -> BoxMenuKeyvarHost<'_>  {
        BoxMenuKeyvarHost {
            map_key: &self.map_key_host,
            var_key: &mut self.var_key_host,
            path: &self.path_var_key,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Page::MenuMain;

        loop {
            match current_box {
                Page::EmptyFullname => current_box = EBOX_EMPTY_FULLNAME.handle(), 
                Page::EmptyHostname => current_box = EBOX_EMPTY_HOSTNAME.handle(), 
                Page::EmptyPasswordRoot => current_box = EBOX_EMPTY_PASSWORD_ROOT.handle(),  
                Page::EmptyPasswordUser => current_box = EBOX_EMPTY_PASSWORD_USER.handle(), 
                Page::EmptyUsername => current_box = EBOX_EMPTY_USERNAME.handle(),
                Page::Finish => current_box = dbox::get_box_message_finish().handle(),
                Page::GaugeInstallation => current_box = Self::get_box_mixed_gauge_installation(self).handle(),
                Page::GaugeTestInstallation => current_box = Self::get_box_mixed_gauge_test_installation(self).handle(),
                Page::InputFullname => current_box = Self::get_box_input_name_full(self).handle(),
                Page::InputHostname => current_box = Self::get_box_input_name_host(self).handle(),
                Page::InputUsergroups => current_box = Self::get_box_input_groups_user(self).handle(),
                Page::InputUsername => current_box = Self::get_box_input_name_user(self).handle(),            
                Page::InvalidHostname => current_box = EBOX_INVALID_HOSTNAME.handle(),
                Page::InvalidUsername => current_box = EBOX_INVALID_USERNAME.handle(), 
                Page::MenuConfig => current_box = Self::get_box_menu_config(self).handle(),
                Page::MenuDevice => current_box = Self::get_box_menu_name_device(self).handle(),
                Page::MenuDrive => current_box = Self::get_box_menu_name_drive(self).handle(),
                Page::MenuMain => current_box = Self::get_box_menu_main(self).handle(),
                Page::MenuKeymapGuest => current_box = Self::get_box_menu_map_key_guest(self).handle(),
                Page::MenuKeymapHost => current_box = Self::get_box_menu_map_key_host(self).handle(),
                Page::MenuKeyvarGuest => current_box = Self::get_box_menu_var_key_guest(self).handle(),
                Page::MenuKeyvarHost => current_box = Self::get_box_menu_var_key_host(self).handle(),
                Page::MenuTimezoneRegion => current_box = Self::get_box_menu_region_timezone(self).handle(),
                Page::MenuTimezoneZone => current_box = Self::get_box_menu_zone_timezone(self).handle(),
                Page::NoMatchPasswordRoot => current_box = EBOX_NOMATCH_PASSWORD_ROOT.handle(),
                Page::NoMatchPasswordUser => current_box = EBOX_NOMATCH_PASSWORD_USER.handle(),
                Page::PasswordRootSgn => current_box = Self::get_box_password_root_sign(self).handle(),
                Page::PasswordRootRpt => current_box = Self::get_box_password_root_repeat(self).handle(),
                Page::PasswordUserSgn => current_box = Self::get_box_password_user_sign(self).handle(),
                Page::PasswordUserRpt => current_box = Self::get_box_password_user_repeat(self).handle(),
                Page::QuestionConfig => current_box = Self::get_box_question_config(self).handle(),
                page => return Self::exit(page, (*self.msg_error).to_string()),
            };
        }
    }
 
    fn exit(page: Page, msg_error: String) -> Result<(), Error> {
        CommandExecute::clear();
        match page {
            Page::EmptyMenu => Err(anyhow!(ErrorInstaller::EmptyMenu())),
            Page::NotFoundBox => Err(anyhow!(ErrorInstaller::NotFoundBox())),
            Page::FailedCommand => Err(anyhow!(ErrorInstaller::FailedCommand(msg_error))),
            Page::Escape => Ok(println!("==> {}", MSG_EXIT_ESCAPE)),
            Page::Quit => Ok(println!("==> {}", MSG_EXIT_QUIT)),
            _ => Err(anyhow!(ErrorInstaller::Unknown(msg_error)))
        }
    }
    
    fn get_builder_list_command(&self) -> ListCommand {
        ListCommand::new(
            &self.name_device, &self.name_user, &self.name_full, 
            &self.password_user, &self.password_root,
            &self.key_pub, Path::new(&format!("/dev/{}", &self.name_drive)), 
            &self.map_key_guest, &self.locale, 
            &self.region_timezone, &self.zone_timezone,
            &self.name_host)
    }
}
