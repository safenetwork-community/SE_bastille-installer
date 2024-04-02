use std::fmt;

use crate::app::dbox::r#type::Page;
use crate::shared::constants::dbox::TITRFOQ;

use dialog::{backends::Dialog, DialogBox};

// Error box dimensions 
const DEFAULT_WIDTH: u32 = 40;
const DEFAULT_HEIGHT: u32 = 10;

// Error box var names
const FULLNAME: &str = "Fullname";
const USERNAME: &str = "Username";
const HOSTNAME: &str = "Hostname";
const PASSWORD_ROOT: &str = "Root password";
const PASSWORD_USER: &str = "User password";

// Error box error text
const ERR_EMPTY: &str = " cannot be empty";
const ERR_INVALID: &str = " contains invalid characters";
const ERR_NOMATCH: &str = " do not match!";

// Error box unwrap failure text 
const EXP_EBOX: &str = "Could not display message box.";

pub struct BoxError<'a> {
    width: u32,
    height: u32,
    text: EboxText<'a>,
    page: Page,
}

impl BoxError<'_> {
    pub fn handle(&self) -> Page {
        let dbox = Dialog::new()
        .set_backtitle(TITRFOQ)
        .set_width(self.width)
        .set_height(self.height);
        dialog::Message::new(self.text.to_string())
            .show_with(dbox)
            .expect(EXP_EBOX);
        self.page.clone()
    }
}

pub struct EboxText<'a> {
    vartype: &'a str,
    errortype: &'a str,
}

impl fmt::Display for EboxText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "{} {}", self.vartype, self.errortype)?;
        writeln!(f, "\n\t\t\tPlease try again")
    }
}

pub const EBOX_EMPTY_FULLNAME: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: FULLNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::InputFullname,
};

pub const EBOX_EMPTY_HOSTNAME: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: HOSTNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::InputHostname,
};

pub const EBOX_EMPTY_PASSWORD_ROOT: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: PASSWORD_ROOT,
        errortype: ERR_EMPTY,
    }, 
    page: Page::PasswordRootSgn,
};

pub const EBOX_EMPTY_PASSWORD_USER: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: PASSWORD_USER,
        errortype: ERR_EMPTY,
    }, 
    page: Page::PasswordUserSgn,
}; 

pub const EBOX_EMPTY_USERNAME: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: USERNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::InputUsername,
}; 

pub const EBOX_INVALID_HOSTNAME: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: HOSTNAME,
        errortype: ERR_INVALID,
    }, 
    page: Page::InputHostname,
}; 

pub const EBOX_INVALID_USERNAME: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: USERNAME,
        errortype: ERR_INVALID,
    }, 
    page: Page::InputUsername,
}; 

pub const EBOX_NOMATCH_PASSWORD_ROOT: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: PASSWORD_ROOT,
        errortype: ERR_NOMATCH,
    }, 
    page: Page::PasswordRootSgn,
}; 

pub const EBOX_NOMATCH_PASSWORD_USER: BoxError = BoxError {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: EboxText{
        vartype: PASSWORD_USER,
        errortype: ERR_NOMATCH,
    }, 
    page: Page::PasswordUserSgn,
};
