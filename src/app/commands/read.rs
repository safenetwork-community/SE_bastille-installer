use std::path::Path;
use std::str; 

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
const ARG_FILTER_MOUNTED_VOL_MAIN: &str = "MOUNTPOINT =~ \"..*\"";
// const ARG_FILTER_MOUNTED_VOLS: &str = "MOUNTPOINTS =~ \"..*\"";
const ARG_FILTER_PTS_ONLY_1: &str = "NAME =~ \"";
const ARG_FILTER_PTS_ONLY_2: &str = "[0-9]*\"";

// Arguments
// const ARG_COL_MOUNTPOINTS: &str = "-no mountpoints -lp";
const ARG_COL_NAME_MOUNTPOINT: &str = "-no name,mountpoint -lp";
const ARG_MOUNTED_VOL_MAIN_1: &str = formatcp!("{ARG_COL_NAME_MOUNTPOINT} --filter '{ARG_FILTER_PTS_ONLY_1}");
const ARG_MOUNTED_VOL_MAIN_2: &str = formatcp!("{ARG_FILTER_PTS_ONLY_2} && {ARG_FILTER_MOUNTED_VOL_MAIN}'");
// const ARG_MOUNTED_VOLS_DEV_1: &str = formatcp!("{ARG_COL_MOUNTPOINTS} --filter '{ARG_FILTER_PTS_ONLY_1}");
// const ARG_MOUNTED_VOLS_DEV_2: &str = formatcp!("{ARG_FILTER_PTS_ONLY_2} && {ARG_FILTER_MOUNTED_VOLS}'");
const ARG_LIST_DRIVES: &str = "-dn -o NAME";

// General programs
const SED: &str = "sed";

// general error messages
const ERR_FAILED_EXECUTE: &str = "Failed to execute";
const ERR_OUT_OF_BOUNDS: &str = "Out of bounds exception";

pub const LIST_MENU_DEVICE: &[[&str; 2]] = &[
    ["Raspberry Pi 4", EMPTY], 
];

pub const LIST_MENU_MAIN: &[(&str, Page)] = &[
    ("Wizard Config", Page::MenuDevice), 
    ("Manual Config", Page::MenuConfig), 
    ("Change keyboard layout", Page::MenuKeymapHost),
    ("Start install", Page::GaugeInstallation),
    ("Test", Page::GaugeTestInstallation),
    ("Quit", Page::Quit)
];

pub const LIST_MENU_CONFIG: &[(&str, Page)] = &[
    ("select device", Page::MenuDevice),
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

    pub fn drives() -> Vec<[String; 2]> {
        
        let command_sh = format!("{LSBLK} {ARG_LIST_DRIVES}"); 
        
        match cmd!(SH, ARG_C, command_sh.clone()).read() {
            Ok(s) => {
                let mut list: Vec<[String; 2]> = Vec::new(); 
                for line in s.lines() {
                list.push([String::from(line),String::new()]); 
                } 
            list
            },
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {command_sh}\n{e}"),
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

    pub fn is_mounted(path_drive: &Path) -> bool {
        let command_sh = format!(r#"{LSBLK} {ARG_MOUNTED_VOL_MAIN_1}{}{ARG_MOUNTED_VOL_MAIN_2}"#, path_drive.display());
        let command = cmd!(SUDO, EOA, SH, ARG_C, command_sh.clone());

        match command.read() {
            Err(e) => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {command_sh}\n{}", e),
            Ok(s) => {
                match s.lines().count() {
                    0 => false,
                    1 => true,
                    integer => panic!("{ERR_FAILED_EXECUTE}: {SUDO} {command_sh}\n{ERR_OUT_OF_BOUNDS}: Max results: 1, Found {}\n{}", integer, s),
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
