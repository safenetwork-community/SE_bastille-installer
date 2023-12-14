mod r#box;
mod dbox;
mod mbox;
mod constants;

use std::process::Command;
use std::path::PathBuf;

use crate::error::Error as Error; 
use crate::error::ErrorKind as ErrorKind; 
use crate::app::r#box::Page;
use crate::app::dbox::*;
use crate::app::dbox::r#type::PageHandler;
use crate::app::mbox::*;

// General box options
pub const CLEAR: &str = "clear";

// Exit texts
pub const MSG_EXIT_ESCAPE: &str =   "Escape pressed, exiting..";
pub const MSG_EXIT_QUIT: &str =     "Quit pressed, exiting..";
pub const MSG_EXIT_FINISH: &str =   "Installation finished! Terminating..";
pub const MSG_EXIT_CONTACT: &str =  "Please contact the owner of this application.";

pub struct App {
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
    error_msg: String,
}

impl App {
    
    pub fn new() -> App {
        App {
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
            error_msg: String::new(),
        }    
    }

    fn get_box_menu_main(&self) -> BoxMenuMain {
        BoxMenuMain {
            error_msg: (&*self.error_msg).to_string(),
        }
    }


    fn get_box_input_username(&self) -> BoxInputUsername {
        BoxInputUsername {
            username: (&*self.username).to_string(),
        }
    }

    fn get_box_input_usergroups(&self) -> BoxInputUsergroups<'_> {
        BoxInputUsergroups {
            username: &*self.username,
            usergroups: (&*self.usergroups).to_string(),
        }
    }

    fn get_box_input_fullname(&self) -> BoxInputFullname<'_> {
        BoxInputFullname {
            username: &*self.username,
            fullname: (&*self.fullname).to_string(),
        }
    }

    fn get_box_password_user_sign(&self) -> BoxPasswordUserSgn<'_> {
        BoxPasswordUserSgn {
            username: &*self.username,
            password_user: (&*self.password_user).to_string(),
        }
    }

    fn get_box_password_user_repeat(&self) -> BoxPasswordUserRpt<'_> {
        BoxPasswordUserRpt {
            username: &*self.username,
            password_user: &*self.password_user,
        }
    }

    fn get_box_password_root_sign(&self) -> BoxPasswordRootSgn {
        BoxPasswordRootSgn {
            password_root: (&*self.password_root).to_string(),
        }
    }

    fn get_box_password_root_repeat(&self) -> BoxPasswordRootRpt {
        BoxPasswordRootRpt {
            password_root: &*self.password_root,
        }
    }

    fn get_box_menu_drive(&self) -> BoxMenuDrive {
        BoxMenuDrive {
            drive: (&*self.drive).to_string(),
        }
    }

    fn get_box_menu_timezone_region(&self) -> BoxMenuTimezoneRegion {
        BoxMenuTimezoneRegion {
            region: (&*self.timezone_region).to_string(),
            pathbuf: (&*self.timezone_path).to_path_buf(),
        }
    }

    fn get_box_menu_timezone_zone(&self) -> BoxMenuTimezoneZone<'_>  {
        BoxMenuTimezoneZone {
            zone: (&*self.timezone_zone).to_string(),
            path: &*self.timezone_path,
        }
    }

    fn get_box_menu_keymap_guest(&self) -> BoxMenuKeymapGuest {
        BoxMenuKeymapGuest {
            keymap: (&*self.keymap_guest).to_string(),
            pathbuf: (&*self.keyvar_path).to_path_buf(),
        }
    }

    fn get_box_menu_keyvar_guest(&self) -> BoxMenuKeyvarGuest<'_>  {
        BoxMenuKeyvarGuest {
            keyvar: (&*self.keyvar_guest).to_string(),
            path: &*self.keyvar_path,
        }
    }

    fn get_box_input_hostname(&self) -> BoxInputHostname {
        BoxInputHostname {
            hostname: (&*self.hostname).to_string(), 
        }
    }
 
    fn get_box_question_confirmation(&self) -> BoxQuestionConfirmation {
        BoxQuestionConfirmation {
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

    fn get_box_menu_config(&self) -> BoxMenuConfig {
        BoxMenuConfig {}
    }

    fn get_box_menu_keymap_host(&self) -> BoxMenuKeymapHost {
        BoxMenuKeymapHost {
            keymap: (&*self.keymap_host).to_string(),
            pathbuf: (&*self.keyvar_path).to_path_buf(),
        }
    }

    fn get_box_menu_keyvar_host(&self) -> BoxMenuKeyvarHost<'_>  {
        BoxMenuKeyvarHost {
            keymap: &*self.keymap_host,
            keyvar: (&*self.keyvar_host).to_string(),
            path: &*self.keyvar_path,
        }
    }


    pub fn run(&mut self) -> Result<(), Error> {
        let mut current_box = Page::MenuMain;

        loop {
            match current_box {
                Page::MenuConfig => current_box = Self::get_box_menu_config(self).handle(),
                Page::MenuMain => current_box = Self::get_box_menu_main(self).handle(),
                Page::Quit => return Self::quit(self),
                Page::Escape => return Self::escape(self),
                Page::Drive => current_box = Self::get_box_menu_drive(self).handle(),
                Page::Fullname => current_box = Self::get_box_input_fullname(self).handle(),
                Page::Hostname => current_box = Self::get_box_input_hostname(self).handle(),
                Page::KeymapGuest => current_box = Self::get_box_menu_keymap_guest(self).handle(),
                Page::KeymapHost => current_box = Self::get_box_menu_keymap_host(self).handle(),
                Page::KeyvarGuest => current_box = Self::get_box_menu_keyvar_guest(self).handle(),
                Page::KeyvarHost => current_box = Self::get_box_menu_keyvar_host(self).handle(),
                Page::PasswordRootSgn => current_box = Self::get_box_password_root_sign(self).handle(),
                Page::PasswordRootRpt => current_box = Self::get_box_password_root_repeat(self).handle(),
                Page::PasswordUserSgn => current_box = Self::get_box_password_user_sign(self).handle(),
                Page::PasswordUserRpt => current_box = Self::get_box_password_user_repeat(self).handle(),
                Page::QuestionConfig => current_box = Self::get_box_question_confirmation(self).handle(),
                Page::TimezoneRegion => current_box = Self::get_box_menu_timezone_region(self).handle(),
                Page::TimezoneZone => current_box = Self::get_box_menu_timezone_zone(self).handle(),
                Page::Usergroups => current_box = Self::get_box_input_usergroups(self).handle(),
                Page::Username => current_box = Self::get_box_input_username(self).handle(),            
                Page::EmptyFullname => current_box = MBOX_EMPTY_FULLNAME.handle(), 
                Page::EmptyHostname => current_box = MBOX_EMPTY_HOSTNAME.handle(), 
                Page::EmptyPasswordRoot => current_box = MBOX_EMPTY_PASSWORD_ROOT.handle(),  
                Page::EmptyPasswordUser => current_box = MBOX_EMPTY_PASSWORD_USER.handle(), 
                Page::EmptyUsername => current_box = MBOX_EMPTY_USERNAME.handle(),
                Page::InvalidHostname => current_box = MBOX_INVALID_HOSTNAME.handle(),
                Page::InvalidUsername => current_box = MBOX_INVALID_USERNAME.handle(), 
                Page::NoMatchPasswordRoot => current_box = MBOX_NOMATCH_PASSWORD_ROOT.handle(),
                Page::NoMatchPasswordUser => current_box = MBOX_NOMATCH_PASSWORD_USER.handle(),
                Page::EmptyMenu => return Self::empty_menu(),
                Page::UnknownError => return Self::unknown_error(self),
                Page::Finish => return Self::finish(self),
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

    fn finish(&mut self) -> Result<(), Error> {
        Self::exit(self, MSG_EXIT_FINISH)
    }

    fn exit(&mut self, msg: &str) -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}", msg);
        Ok(())
    }

    fn box_not_found() -> Result<(), Error> {
        Command::new(CLEAR).status().unwrap();
        eprintln!("==> {}\n{}", Error::new(ErrorKind::BoxNotFound()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::BoxNotFound()))    
    }
 
    fn empty_menu() -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}\n{}", Error::new(ErrorKind::EmptyMenu()), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::EmptyMenu()))    
    }

    fn unknown_error(&mut self) -> Result<(), Error> {     
        Command::new(CLEAR).status().unwrap();
        println!("==> {}\n{}", Error::new(ErrorKind::UnknownError(self.error_msg.clone())), MSG_EXIT_CONTACT);
        Err(Error::new(ErrorKind::UnknownError(self.error_msg.clone())))    
    }    
}
