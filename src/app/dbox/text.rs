use std::fmt;

/*
pub struct MainMenuText {       
}

impl fmt::Display for MainMenuText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to do")
    }
}

pub struct UsernameText {       
}

impl fmt::Display for UsernameText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter the username you want:")?;
        writeln!(f, "(usernames must be all lowercase and first character may not be a number)")
    }
}

pub struct UsergroupsText<'a> {       
    pub username: &'a str,
}

impl fmt::Display for UsergroupsText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter additional groups besides the default groups which are")?;
        writeln!(f, "--> wheel,sys,audio,input,video,storage,lp,network,users,power <--")?;
        writeln!(f, "for user '{}' in a comma seperated list:", self.username) 
    }
}

pub struct FullnameText<'a> {       
    pub username: &'a str,
}

impl fmt::Display for FullnameText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter desired full name for {}:", self.username)
    }
}

pub struct PasswordUserSignText<'a> {       
    pub username: &'a str,
}

impl fmt::Display for PasswordUserSignText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter new password for {}:", self.username)
    }
}

pub struct PasswordUserRepeatText<'a> {      
    pub username: &'a str,
}

impl fmt::Display for PasswordUserRepeatText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Confirm password for {}:", self.username)
    }
}

pub struct PasswordRootSignText {        
}

impl fmt::Display for PasswordRootSignText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter new root password:")
    }
}

pub struct PasswordRootRepeatText {
}

impl fmt::Display for PasswordRootRepeatText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Confirm new root password:")
    }
}

pub struct DriveText {
}

impl fmt::Display for DriveText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your SDCard/eMMC/USB - Be sure the correct drive is selected!")?;
        writeln!(f, "WARNING! This WILL destroy the data on it!")
    }
}

pub struct KeymapGuestText {
}

impl fmt::Display for KeymapGuestText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard layout:")
    }
}

pub struct KeyvarGuestText {
}

impl fmt::Display for KeyvarGuestText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard variant:")
    }
}

pub struct TimezoneRegionText {
}

impl fmt::Display for TimezoneRegionText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone region:")
    }
}

pub struct TimezoneZoneText {
}

impl fmt::Display for TimezoneZoneText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone:")
    }
}

pub struct HostnameText {
}

impl fmt::Display for HostnameText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter desired hostname for this system:")
    }
}
*/
pub struct ConfirmationText<'a> {
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


impl fmt::Display for ConfirmationText<'_> {
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
/*
pub struct KeymapHostText {
}

impl fmt::Display for KeymapHostText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard layout for this installer:")
    }
}

pub struct KeyvarHostText {
}

impl fmt::Display for KeyvarHostText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard variant for this installer:")
    }
}


pub struct MenuConfigText {
}

impl fmt::Display for MenuConfigText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to reconfigure:")
    }
}
*/
