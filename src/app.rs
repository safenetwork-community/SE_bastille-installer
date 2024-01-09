mod dbox;
mod constants;

use std::path::PathBuf;

use crate::error::Error as Error; 
use crate::error::ErrorKind as ErrorKind; 
use crate::app::dbox::*;
use crate::app::dbox::command::*;
use crate::app::dbox::ebox::*;
use crate::app::dbox::r#type::*;

// Exit texts
pub const MSG_EXIT_ESCAPE: &str =   "Escape pressed, exiting..";
pub const MSG_EXIT_QUIT: &str =     "Quit pressed, exiting..";
pub const MSG_EXIT_CONTACT: &str =  "Please contact the owner of this application.";

pub struct App {
    device: String,
    os: String,
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
    timezone_region: String,
    timezone_zone: String,
    hostname: String,
    single_edit: bool,
    error_msg: String,
    dbox: DBoxPool
}

impl App {
    
    pub fn new() -> Self {
        App {
            device: String::new(),
            os: String::new(), 
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
            timezone_region: String::new(),
            timezone_zone: String::new(),
            usergroups: String::new(),
            hostname: String::new(),
            single_edit: false,
            error_msg: String::new(),
            dbox: DBoxPool::new(),
        }    
    }
    
    fn get_box_menu_main(&mut self) -> BoxMenuMain<'_> {
        BoxMenuMain {
            error_msg: &mut self.error_msg,
        }
    }


    fn get_box_input_username(&mut self) -> BoxInputUsername<'_> {
        BoxInputUsername {
            single_edit: self.single_edit,
            username: &mut self.username,
        }
    }

    fn get_box_input_usergroups(&mut self) -> BoxInputUsergroups<'_> {
        BoxInputUsergroups {
            single_edit: self.single_edit,
            username: &*self.username,
            usergroups: &mut self.usergroups,
        }
    }

    fn get_box_input_fullname(&mut self) -> BoxInputFullname<'_> {
        BoxInputFullname {
            single_edit: self.single_edit,
            username: &*self.username,
            fullname: &mut self.fullname,
        }
    }

    fn get_box_password_user_sign(&mut self) -> BoxPasswordUserSgn<'_> {
        BoxPasswordUserSgn {
            single_edit: self.single_edit,
            username: &*self.username,
            password_user: &mut self.password_user,
        }
    }

    fn get_box_password_user_repeat(&self) -> BoxPasswordUserRpt<'_> {
        BoxPasswordUserRpt {
            single_edit: self.single_edit,
            username: &*self.username,
            password_user: &*self.password_user,
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
            password_root: &*self.password_root,
        }
    }

    fn get_box_menu_device(&mut self) -> BoxMenuDevice {
        BoxMenuDevice {
            device: &mut self.device,
            single_edit: self.single_edit,
        }
    }

    fn get_box_menu_drive(&mut self) -> BoxMenuDrive {
        BoxMenuDrive {
            drive: &mut self.drive,
            single_edit: self.single_edit,
        }
    }

    fn get_box_menu_os(&mut self) -> BoxMenuOperatingSystem {
        BoxMenuOperatingSystem {
            os: &mut self.os,
            single_edit: self.single_edit,
        }
    }

    fn get_box_menu_timezone_region(&mut self) -> BoxMenuTimezoneRegion {
        BoxMenuTimezoneRegion {
            single_edit: self.single_edit,
            region: &mut self.timezone_region,
            pathbuf: &mut self.timezone_path,
        }
    }

    fn get_box_menu_timezone_zone(&mut self) -> BoxMenuTimezoneZone<'_>  {
        BoxMenuTimezoneZone {
            single_edit: self.single_edit,
            zone: &mut self.timezone_zone,
            path: &*self.timezone_path,
        }
    }

    fn get_box_menu_keymap_guest(&mut self) -> BoxMenuKeymapGuest {
        BoxMenuKeymapGuest {
            single_edit: self.single_edit,
            keymap: &mut self.keymap_guest,
            pathbuf: &mut self.keyvar_path,
        }
    }

    fn get_box_menu_keyvar_guest(&mut self) -> BoxMenuKeyvarGuest<'_>  {
        BoxMenuKeyvarGuest {
            single_edit: self.single_edit,
            keyvar: &mut self.keyvar_guest,
            path: &*self.keyvar_path,
        }
    }

    fn get_box_input_hostname(&mut self) -> BoxInputHostname {
        BoxInputHostname {
            single_edit: self.single_edit,
            hostname: &mut self.hostname, 
        }
    }
 
    fn get_box_question_config(&self) -> BoxQuestionConfig {
        BoxQuestionConfig {
            username: &*self.username,
            fullname: &*self.fullname,
            usergroups: &*self.usergroups,
            drive: &*self.drive, 
            timezone_region: &*self.timezone_region,
            timezone_zone: &*self.timezone_zone,
            keymap: &*self.keymap_guest,
            keyvar: &*self.keyvar_guest,
            hostname: &*self.hostname,
        }
    }

    fn get_box_mixed_gauge_installation(&self) -> BoxGaugeInstallation {

        let list_installation: &[(&str, Box<dyn Fn()>)] = &[
            ("Show elapsing time..", Box::new(CommandInstall::show_elapsed_time)),
            ("Creating install..", Box::new(CommandInstall::create_install))
        ];

        BoxGaugeInstallation {
            username: &self.username,
            fullname: &self.fullname,
            usergroups: &self.usergroups,
            drive: &self.drive, 
            timezone_region: &self.timezone_region,
            timezone_zone: &self.timezone_zone,
            keymap: &self.keymap_guest,
            keyvar: &self.keyvar_guest,
            hostname: &self.hostname,
            list_installation: list_installation,
        }
    }


    fn get_box_menu_config(&mut self) -> BoxMenuConfig {
        BoxMenuConfig {
            single_edit: &mut self.single_edit,
        }
    }

    fn get_box_menu_keymap_host(&mut self) -> BoxMenuKeymapHost<'_> {
        BoxMenuKeymapHost {
            keymap: &mut self.keymap_host,
            pathbuf: &mut self.keyvar_path,
        }
    }

    fn get_box_menu_keyvar_host(&mut self) -> BoxMenuKeyvarHost<'_>  {
        BoxMenuKeyvarHost {
            keymap: &*self.keymap_host,
            keyvar: &mut self.keyvar_host,
            path: &*self.keyvar_path,
        }
    }


    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Page::MenuMain;

        loop {
            match current_box {
                Page::Escape => return Self::escape(self),
                Page::EmptyMenu => return Self::empty_menu(),
                Page::EmptyFullname => current_box = EBOX_EMPTY_FULLNAME.handle(), 
                Page::EmptyHostname => current_box = EBOX_EMPTY_HOSTNAME.handle(), 
                Page::EmptyPasswordRoot => current_box = EBOX_EMPTY_PASSWORD_ROOT.handle(),  
                Page::EmptyPasswordUser => current_box = EBOX_EMPTY_PASSWORD_USER.handle(), 
                Page::EmptyUsername => current_box = EBOX_EMPTY_USERNAME.handle(),
                Page::Finish => current_box = self.dbox.message_finish.handle(),
                Page::InputFullname => current_box = Self::get_box_input_fullname(self).handle(),
                Page::InputHostname => current_box = Self::get_box_input_hostname(self).handle(),
                Page::InputUsergroups => current_box = Self::get_box_input_usergroups(self).handle(),
                Page::InputUsername => current_box = Self::get_box_input_username(self).handle(),            
                Page::InvalidHostname => current_box = EBOX_INVALID_HOSTNAME.handle(),
                Page::InvalidUsername => current_box = EBOX_INVALID_USERNAME.handle(), 
                Page::MenuConfig => current_box = Self::get_box_menu_config(self).handle(),
                Page::MenuDevice => current_box = Self::get_box_menu_device(self).handle(),
                Page::MenuDrive => current_box = Self::get_box_menu_drive(self).handle(),
                Page::MenuMain => current_box = Self::get_box_menu_main(self).handle(),
                Page::MenuKeymapGuest => current_box = Self::get_box_menu_keymap_guest(self).handle(),
                Page::MenuKeymapHost => current_box = Self::get_box_menu_keymap_host(self).handle(),
                Page::MenuKeyvarGuest => current_box = Self::get_box_menu_keyvar_guest(self).handle(),
                Page::MenuKeyvarHost => current_box = Self::get_box_menu_keyvar_host(self).handle(),
                Page::MenuOperatingSystem => current_box = Self::get_box_menu_os(self).handle(),
                Page::MenuTimezoneRegion => current_box = Self::get_box_menu_timezone_region(self).handle(),
                Page::MenuTimezoneZone => current_box = Self::get_box_menu_timezone_zone(self).handle(),
                Page::GaugeInstallation => current_box = Self::get_box_mixed_gauge_installation(self).handle(),
                Page::NoMatchPasswordRoot => current_box = EBOX_NOMATCH_PASSWORD_ROOT.handle(),
                Page::NoMatchPasswordUser => current_box = EBOX_NOMATCH_PASSWORD_USER.handle(),
                Page::PasswordRootSgn => current_box = Self::get_box_password_root_sign(self).handle(),
                Page::PasswordRootRpt => current_box = Self::get_box_password_root_repeat(self).handle(),
                Page::PasswordUserSgn => current_box = Self::get_box_password_user_sign(self).handle(),
                Page::PasswordUserRpt => current_box = Self::get_box_password_user_repeat(self).handle(),
                Page::QuestionConfig => current_box = Self::get_box_question_config(self).handle(),
                Page::Quit => return Self::quit(self),
                Page::UnknownError => return Self::unknown_error(self),
                _ => return Self::box_not_found(),
            };
        }
    }
    
    fn quit(&mut self) -> Result<(), Error> {      
        Self::exit(self, MSG_EXIT_QUIT)
    }
 
    fn escape(&mut self) -> Result<(), Error> {     
       Self::exit(self, MSG_EXIT_ESCAPE)
    }

    fn exit(&mut self, msg: &str) -> Result<(), Error> {     
        CommandInstall::clear();
        println!("==> {}", msg);
        Ok(())
    }

    fn box_not_found() -> Result<(), Error> {
        CommandInstall::clear();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }
 
    fn empty_menu() -> Result<(), Error> {     
        CommandInstall::clear();
        println!("==> {}\n{}", Error::new(ErrorKind::EmptyMenu()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::EmptyMenu()))    
    }

    fn unknown_error(&mut self) -> Result<(), Error> {     
        CommandInstall::clear();
        println!("==> {}\n{}", Error::new(ErrorKind::UnknownError(self.error_msg.clone())), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::UnknownError(self.error_msg.clone())))    
    }    
}
