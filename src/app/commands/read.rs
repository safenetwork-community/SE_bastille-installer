use std::path::Path;

use const_format::formatcp;
use duct::cmd;

use crate::app::dbox::r#type::Page;
use crate::shared::constants::command::*;
use crate::shared::constants::string::EMPTY;

// find regex
pub const REGEX_FIND_DIRS_ALL: &str = r". -regex \.\/[^\.].* -prune -type d";
pub const REGEX_FIND_DIRS_CAP: &str = r". -regex \.\/[A-Z].* -prune -type d";
pub const REGEX_FIND_FILES: &str = r". -regex \.\/[^\.].* -prune -type f";

// sed regex
pub const SED_FIND_DEFAULT: &str = r"s/\.\///";
pub const SED_FIND_FILE_EXTENSIONS: &str = r"s/\.\/\([^\.]*\).*/\1/";

// Paths
pub const PATH_BKEYMAP: &str = "/usr/share/bkeymaps";  
pub const PATH_ZONEINFO: &str = "/usr/share/zoneinfo";  

// Scols-filter arguments
const ARG_FILTER_MOUNTED: &str = r#"MOUNTPOINT =~ '..*'"#;
const ARG_FILTER_NAME: &str = r#"NAME =~"#;
const ARG_FILTER_TYPE_DISK: &str = formatcp!("&& TYPE =~ '{ACS_DISK}'");
const ARG_FILTER_TYPE_PART: &str = formatcp!("&& TYPE =~ '{ACS_PART}'");
const ARG_FILTER_E_PTS_MTD: &str = formatcp!("{ARG_FILTER_TYPE_PART} && {ARG_FILTER_MOUNTED}");

const ACS_NAME: &str = "NAME";
const ARG_NAME_MPT: &str = "name,mountpoint";

// General programs
const SED: &str = "sed";

// general error messages
const ERR_FAILED_EXECUTE: &str = "Failed to execute";
const ERR_OUT_OF_BOUNDS: &str = "Out of bounds exception";

pub const LIST_MENU_DEVICE: &[[&str; 2]] = &[
    ["Raspberry Pi 4", EMPTY], 
];

pub const LIST_MENU_MAIN: &[(&str, Page)] = &[
    ("Wizard Config", Page::MenuServerBoard), 
    ("Manual Config", Page::MenuConfig), 
    ("Change keyboard layout", Page::MenuKeymapHost),
    ("Start install", Page::GaugeInstallation),
    ("Test", Page::GaugeTestInstallation),
    ("Quit", Page::Quit)
];

pub const LIST_MENU_CONFIG: &[(&str, Page)] = &[
    ("select device", Page::MenuServerBoard),
    ("enter username", Page::InputUsername),
    ("enter usergroups", Page::InputUsergroups),
    ("enter fullname", Page::InputFullname),
    ("set password of user", Page::PasswordUserSgn),
    ("set password of root", Page::PasswordRootSgn),
    ("select drive", Page::MenuDrive),
    ("select timezone", Page::MenuTimezoneRegion),
    ("select keyboard layout", Page::MenuKeymapGuest),
    ("enter hostname", Page::InputHostname)
];

pub struct CommandRead {}

impl CommandRead {

    pub fn mountpoints_drive(drive: &Path) -> Result<String, std::io::Error> {
        let filter = format!(r#"{ARG_FILTER_NAME} '{}' {ARG_FILTER_E_PTS_MTD}"#, 
            drive.file_name().unwrap().to_str().unwrap());
        let command = cmd!(LSBLK, ARG_LN, ARG_O, ACS_MOUNTPOINTS, ARL_FILTER, filter);
        command.stderr_capture().unchecked().read()
    }

    pub fn partitions_drive(drive: &str) -> Vec<[String; 2]> {
        let filter = format!(r#"{ARG_FILTER_NAME} '{}' {ARG_FILTER_TYPE_PART}"#, drive);
        let command = cmd!(LSBLK, ARG_LN, ARG_O, ACS_NAME, ARL_FILTER, filter);

        match command.stderr_capture().unchecked().read() {
            Ok(s) => {
                let mut list: Vec<[String; 2]> = Vec::new(); 
                for line in s.lines() {
                list.push([String::from(drive), String::from(line.strip_prefix(drive).unwrap())]); 
                } 
            list
            },
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {:?}\n{e}", command),
        }

    }

    pub fn drives() -> Vec<[String; 2]> {
        
        let command = cmd!(LSBLK, ARG_DN, ARG_O, ACS_NAME); 
        
        match command.read() {
            Ok(s) => {
                let mut list: Vec<[String; 2]> = Vec::new(); 
                for line in s.lines() {
                list.push([String::from(line),String::new()]); 
                } 
            list
            },
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {:?}\n{e}", command),
        }
    }

    pub fn drive_exists(drive: &Path) -> bool {
        let filter = format!(r#"{ARG_FILTER_NAME} '{}' {ARG_FILTER_TYPE_DISK}"#, drive.file_name().unwrap().to_str().unwrap());        
        let command = cmd!(LSBLK, ARG_DN, ARG_O, ACS_NAME, ARL_FILTER, filter);

        match command.read() {
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {:?}\n{}", command, e),
            Ok(s) => {
                match s.lines().count() {
                    0 => false,
                    1 => true,
                    integer => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {:?}\n{ERR_OUT_OF_BOUNDS}: Max results: 1, Found {}\n{}", command, integer, s),
                }
            },
        }
    }

    pub fn keyvars(path: &Path) -> Vec<[String; 2]> {
        Self::find(path, 
            REGEX_FIND_FILES, SED_FIND_FILE_EXTENSIONS)
    }

    pub fn keymap() -> Vec<[String; 2]> {
        Self::find(Path::new(PATH_BKEYMAP), 
            REGEX_FIND_DIRS_ALL, SED_FIND_DEFAULT)
    }

    pub fn is_mounted(drive: &str) -> bool {
        let filter = format!(r#"{ARG_FILTER_NAME} '{}' {ARG_FILTER_E_PTS_MTD}"#, drive);
        let command = cmd!(LSBLK, ARG_NO, ARG_NAME_MPT, ARG_LP, ARL_FILTER, filter);

        match command.read() {
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {:?}\n{}", command, e),
            Ok(s) => {
                match s.lines().count() {
                    0 => false,
                    1 => true,
                    integer => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {:?}\n{ERR_OUT_OF_BOUNDS}: Max results: 1, Found {}\n{}", command, integer, s),
                }
            },
        }
    }

    pub fn timeregion() -> Vec<[String; 2]> {
        Self::find(Path::new(PATH_ZONEINFO), 
            REGEX_FIND_DIRS_CAP, SED_FIND_DEFAULT)
    }

    pub fn timezone(path: &Path) -> Vec<[String; 2]> {
        Self::find(path, 
            REGEX_FIND_FILES, SED_FIND_DEFAULT)
    }

    fn find(path: &Path, regex_find: &str, regex_sed: &str) -> Vec<[String; 2]> {
        let command_sh = format!("find {}", regex_find);
        match cmd!(SH, ARG_C, command_sh.clone()).pipe(cmd!(SED, regex_sed)).dir(path).read() {
            Err(e) => panic!("{ERR_FAILED_EXECUTE} {command_sh} | {SED} {regex_sed}\n{e}"),
            Ok(s) => {
                let mut list: Vec<[String; 2]> = Vec::new(); 
                for line in s.lines() {
                list.push([String::from(line),String::new()]); 
                } 
            list
            },
        }
    } 
}
