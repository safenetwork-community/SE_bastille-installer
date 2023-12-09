use std::fmt;

pub struct MainMenuForm {       
}

impl fmt::Display for MainMenuForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to do")
    }
}

pub struct UsernameForm {       
}

impl fmt::Display for UsernameForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter the username you want:")?;
        writeln!(f, "(usernames must be all lowercase and first character may not be a number)")
    }
}

pub struct UsergroupsForm<'a> {       
    pub username: &'a str,
}

impl fmt::Display for UsergroupsForm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter additional groups besides the default groups which are")?;
        writeln!(f, "--> wheel,sys,audio,input,video,storage,lp,network,users,power <--")?;
        writeln!(f, "for user '{}' in a comma seperated list:", self.username) 
    }
}

pub struct FullnameForm<'a> {       
    pub username: &'a str,
}

impl fmt::Display for FullnameForm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter desired full name for {}:", self.username)
    }
}

pub struct PasswordUserSignForm<'a> {       
    pub username: &'a str,
}

impl fmt::Display for PasswordUserSignForm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Enter new password for {}:", self.username)
    }
}

pub struct PasswordUserRepeatForm<'a> {      
    pub username: &'a str,
}

impl fmt::Display for PasswordUserRepeatForm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "Confirm password for {}:", self.username)
    }
}

pub struct PasswordRootSignForm {        
}

impl fmt::Display for PasswordRootSignForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter new root password:")
    }
}

pub struct PasswordRootRepeatForm {
}

impl fmt::Display for PasswordRootRepeatForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Confirm new root password:")
    }
}

pub struct DriveForm {
}

impl fmt::Display for DriveForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your SDCard/eMMC/USB - Be sure the correct drive is selected!")?;
        writeln!(f, "WARNING! This WILL destroy the data on it!")
    }
}

pub struct KeymapGuestForm {
}

impl fmt::Display for KeymapGuestForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard layout:")
    }
}

pub struct KeyvarGuestForm {
}

impl fmt::Display for KeyvarGuestForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your desired keyboard variant:")
    }
}

pub struct TimezoneRegionForm {
}

impl fmt::Display for TimezoneRegionForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone region:")
    }
}

pub struct TimezoneZoneForm {
}

impl fmt::Display for TimezoneZoneForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose your timezone:")
    }
}

pub struct HostnameForm {
}

impl fmt::Display for HostnameForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Enter desired hostname for this system:")
    }
}

pub struct ConfirmationForm<'a> {
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


impl fmt::Display for ConfirmationForm<'_> {
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

pub struct KeymapHostForm {
}

impl fmt::Display for KeymapHostForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard layout for this installer:")
    }
}

pub struct KeyvarHostForm {
}

impl fmt::Display for KeyvarHostForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "Choose a keyboard variant for this installer:")
    }
}


pub struct MenuConfigForm {
}

impl fmt::Display for MenuConfigForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        writeln!(f, "What would you like to reconfigure:")
    }
}


