use std::fmt;

use crate::app::constants::*;
use crate::app::r#box::Page;

use dialog::{backends::Dialog, DialogBox};

// Message box dimensions 
const DEFAULT_WIDTH: u32 = 40;
const DEFAULT_HEIGHT: u32 = 10;

// Message box var names
const FULLNAME: &str = "Fullname";
const USERNAME: &str = "Username";
const HOSTNAME: &str = "Hostname";
const PASSWORD_ROOT: &str = "Root password";
const PASSWORD_USER: &str = "User password";

// Message box error text
const ERR_EMPTY: &str = " cannot be empty";
const ERR_INVALID: &str = " contains invalid characters";
const ERR_NOMATCH: &str = " do not match!";

// Massage box unwrap failure text 
const EXP_MBOX: &str = "Could not display message box.";

pub struct MessageBox<'a> {
    width: u32,
    height: u32,
    text: MboxText<'a>,
    page: Page,
}

impl MessageBox<'_> {
    pub fn handle(&self) -> Page {
        let mut dbox = Dialog::new();
        dbox.set_backtitle(TITRFOQ);
        dbox.set_width(self.width);
        dbox.set_height(self.height);
        dialog::Message::new(self.text.to_string())
            .show_with(&dbox)
            .expect(EXP_MBOX);
        self.page.clone()
    }
}

pub struct MboxText<'a> {
    vartype: &'a str,
    errortype: &'a str,
}

impl fmt::Display for MboxText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "{} {}", self.vartype, self.errortype)?;
        writeln!(f, "\n\t\t\tPlease try again")
    }
}

pub const MBOX_EMPTY_FULLNAME: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: FULLNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::Fullname,
};

pub const MBOX_EMPTY_HOSTNAME: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: HOSTNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::Hostname,
};

pub const MBOX_EMPTY_PASSWORD_ROOT: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: PASSWORD_ROOT,
        errortype: ERR_EMPTY,
    }, 
    page: Page::PasswordRootSgn,
};

pub const MBOX_EMPTY_PASSWORD_USER: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: PASSWORD_USER,
        errortype: ERR_EMPTY,
    }, 
    page: Page::PasswordUserSgn,
}; 

pub const MBOX_EMPTY_USERNAME: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: USERNAME,
        errortype: ERR_EMPTY,
    }, 
    page: Page::Username,
}; 

pub const MBOX_INVALID_HOSTNAME: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: HOSTNAME,
        errortype: ERR_INVALID,
    }, 
    page: Page::Hostname,
}; 

pub const MBOX_INVALID_USERNAME: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: USERNAME,
        errortype: ERR_INVALID,
    }, 
    page: Page::Username,
}; 

pub const MBOX_NOMATCH_PASSWORD_ROOT: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: PASSWORD_ROOT,
        errortype: ERR_NOMATCH,
    }, 
    page: Page::PasswordRootSgn,
}; 

pub const MBOX_NOMATCH_PASSWORD_USER: MessageBox = MessageBox {
    width: DEFAULT_WIDTH, 
    height: DEFAULT_HEIGHT, 
    text: MboxText{
        vartype: PASSWORD_USER,
        errortype: ERR_NOMATCH,
    }, 
    page: Page::PasswordUserSgn,
}; 
