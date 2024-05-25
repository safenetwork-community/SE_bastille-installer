use std::fmt;

use crate::shared::constants::install::DEFAULT_USERGROUPS;

pub struct TextGaugeInstallation<'a> {    
    pub functions: &'a [(); 4],
}

impl fmt::Display for TextGaugeInstallation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.functions)?;
        writeln!(f, "Progress..")
    }
}

pub struct TextMenuMain {       
}

impl fmt::Display for TextMenuMain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to do")
    }
}

pub struct TextMenuDevice {       
}

impl fmt::Display for TextMenuDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a device you want to install SE Bastille on:")
    }
}

pub struct TextInputUsername {       
}

impl fmt::Display for TextInputUsername {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter the username you want:")?;
        writeln!(f, "(usernames must be all lowercase and first character may not be a number)")
    }
}

pub struct TextInputUsergroups<'a> {       
    pub username: &'a str,
}

impl fmt::Display for TextInputUsergroups<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter additional groups besides the default groups which are")?;
        writeln!(f, "--> {DEFAULT_USERGROUPS} <--")?;
        writeln!(f, "for user '{}' in a comma seperated list:", self.username) 
    }
}

pub struct TextInputFullname<'a> {       
    pub username: &'a str,
}

impl fmt::Display for TextInputFullname<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter desired full name for {}:", self.username)
    }
}

pub struct TextPasswordUserSgn<'a> {       
    pub username: &'a str,
}

impl fmt::Display for TextPasswordUserSgn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter new password for {}:", self.username)
    }
}

pub struct TextPasswordUserRpt<'a> {      
    pub username: &'a str,
}

impl fmt::Display for TextPasswordUserRpt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Confirm password for {}:", self.username)
    }
}

pub struct TextPasswordRootSgn {        
}

impl fmt::Display for TextPasswordRootSgn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter new root password:")
    }
}

pub struct TextPasswordRootRpt {
}

impl fmt::Display for TextPasswordRootRpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Confirm new root password:")
    }
}

pub struct TextMenuDrive {
}

impl fmt::Display for TextMenuDrive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your SDCard/eMMC/USB - Be sure the correct drive is selected!")?;
        writeln!(f, "WARNING! This WILL destroy the data on it!")
    }
}

pub struct TextMenuKeymapGuest {
}

impl fmt::Display for TextMenuKeymapGuest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard layout:")
    }
}

pub struct TextMenuKeyvarGuest {
}

impl fmt::Display for TextMenuKeyvarGuest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard variant:")
    }
}

pub struct TextMenuTimezoneRegion {
}

impl fmt::Display for TextMenuTimezoneRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone region:")
    }
}

pub struct TextMenuTimezoneZone {
}

impl fmt::Display for TextMenuTimezoneZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone:")
    }
}

pub struct TextInputHostname {
}

impl fmt::Display for TextInputHostname {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter desired hostname for this system:")
    }
}

pub struct TextQuestionConfig<'a> {
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


impl fmt::Display for TextQuestionConfig<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Is the below informate correct:")?;
        writeln!(f, "Username: {}", self.username)?;
        writeln!(f, "Full Username: {}", self.fullname)?;
        writeln!(f, "Additional usergroup = {}", self.usergroups)?;
        writeln!(f, "Password for {} = (password hidden)", self.username)?;
        writeln!(f, "Password for root = (password hidden)")?;
        writeln!(f, "SDCard/eMMC/USB = {}", self.drive)?;
        writeln!(f, "Timezone = {}/{}", self.timezone_region, self.timezone_zone)?;
        writeln!(f, "TTY Keyboard layout = {}/{}", self.keymap, self.keyvar)?;
        writeln!(f, "Hostname = {}", self.hostname)
    }
}

pub struct TextMenuKeymapHost {
}

impl fmt::Display for TextMenuKeymapHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard layout for this installer:")
    }
}

pub struct TextMenuKeyvarHost {
}

impl fmt::Display for TextMenuKeyvarHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard variant for this installer:")
    }
}


pub struct TextMenuConfig {
}

impl fmt::Display for TextMenuConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to reconfigure:")
    }
}

