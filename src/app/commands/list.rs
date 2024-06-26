use std::path::Path;
use std::process::{Command, Stdio};
use std::str; 

use const_format::str_split;

use crate::app::dbox::r#type::Page;
use crate::shared::constants::char::SPACE;
use crate::shared::constants::command::{ARG_C, LSBLK, SH};
use crate::shared::constants::string::EMPTY;

// find regex
pub const REGEX_FIND_DIRS_ALL: [&str; 6] = [".","-regex",r"\.\/[^\.].*","-prune","-type","d"];
pub const REGEX_FIND_DIRS_CAP: [&str; 6] = [".","-regex",r"\.\/[A-Z].*","-prune","-type","d"];
pub const REGEX_FIND_FILES: [&str; 6] = [".","-regex",r"\.\/[^\.].*","-prune","-type","f"];

// sed regex
pub const SED_FIND_DEFAULT: &str = r"s/\.\///";
pub const SED_FIND_FILE_EXTENSIONS: &str = r"s/\.\/\([^\.]*\).*/\1/";

// Paths
pub const PATH_BKEYMAP: &str = "/usr/share/bkeymaps";  
pub const PATH_ZONEINFO: &str = "/usr/share/zoneinfo";  

// Arguments
const ARG_MOUNTED_PARTITIONS: &str = "-no name,mountpoints -lp";
const ARG_FILTER_MOUNTPOINT: &str = "--filter 'MOUNTPOINT'";
const ARG_ALL_PARTITIONS: &str = "-no name -lp";
const ARG_LIST_DRIVES: &str = "-dn -o NAME";
const ARGS_LIST_DRIVES: [&str; 3] = str_split!(ARG_LIST_DRIVES, SPACE);

// General programs
const FIND: &str = "find";
const SED: &str = "sed";

// general error messages
const ERR_FAILED_EXECUTE_LSBLK: &str = "Failed to execute lsblk";
const ERR_FAILED_EXECUTE_FIND: &str = "Failed to execute find";
const ERR_FAILED_EXECUTE_SED: &str = "Failed to execute sed";
const ERR_FAILED_WAIT_SED: &str = "Failed to wait on sed process";

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

pub struct ListFromCommand {}

impl ListFromCommand {
    pub fn drives() -> Vec<[String; 2]> {
        let output_command = Command::new(LSBLK)
        .args(ARGS_LIST_DRIVES)
        .output()
        .unwrap_or_else(|e| panic!("{}{}\n{}", ERR_FAILED_EXECUTE_LSBLK, ARG_LIST_DRIVES, e));
 
        let mut list: Vec<[String; 2]> = Vec::new(); 
        let stdout_command = String::from_utf8_lossy(&output_command.stdout);
        for line in stdout_command.lines() {
           list.push([String::from(line),String::new()]); 
        } 
        list
    }

    pub fn keyvars(path: &Path) -> Vec<[String; 2]> {
        Self::find(path, 
            REGEX_FIND_FILES.to_vec(), SED_FIND_FILE_EXTENSIONS)
    }

    pub fn keymap() -> Vec<[String; 2]> {
        Self::find(Path::new(PATH_BKEYMAP), 
            REGEX_FIND_DIRS_ALL.to_vec(), SED_FIND_DEFAULT)
    }

    pub fn mounted_all() -> Vec<String> {
        let command_sh = format!("{} {} {}", LSBLK, ARG_MOUNTED_PARTITIONS, ARG_FILTER_MOUNTPOINT);
        let output_command = Command::new(SH)
            .arg(ARG_C)
            .arg(command_sh.clone()) 
            .output()
            .unwrap_or_else(|e| panic!("{}{}\n{}", ERR_FAILED_EXECUTE_LSBLK, command_sh, e));

        match String::from_utf8(output_command.stdout) {
            Ok(output) => { 
                output.lines().map(|e| { 
                String::from(e.trim())
                }).collect()
            }
            _ => panic!("UtfError partitions mounted"),
        } 
    }

    pub fn mounted_partitions(path_drive: &Path) -> Vec<String> {
        let command_sh = format!(r#"{} {} {}? {}"#, LSBLK, ARG_MOUNTED_PARTITIONS, path_drive.display(), ARG_FILTER_MOUNTPOINT);
        let output_command = Command::new(SH)
            .arg(ARG_C)
            .arg(command_sh.clone()) 
            .output()
            .unwrap_or_else(|e| panic!("{}{}\n{}", ERR_FAILED_EXECUTE_LSBLK, command_sh, e));

        match String::from_utf8(output_command.stdout) {
            Ok(output) => { 
                output.lines().map(|e| { 
                String::from(e.trim())
                }).collect()
            }
            _ => panic!("UtfError partitions mounted"),
        } 
    }

    pub fn partition_numbers(drive: &Path) -> Vec<String> {
        let command_sh = format!(r#"{} {} {}?"#, LSBLK, ARG_ALL_PARTITIONS, drive.display());
        let output_command = Command::new(SH)
            .arg(ARG_C)
            .arg(command_sh.clone())
            .output()
            .unwrap_or_else(|e| panic!("{}{}\n{}", ERR_FAILED_EXECUTE_LSBLK, command_sh, e));
       
        match String::from_utf8(output_command.stdout) {
            Ok(output) => {
                output.lines().map(|e| {
                    String::from(e.trim().strip_prefix(drive.to_str().unwrap()).unwrap())
                }).collect()
            }
            _ => panic!("UtfError partitions mounted"),
        }
    }

    pub fn timeregion() -> Vec<[String; 2]> {
        Self::find(Path::new(PATH_ZONEINFO), 
            REGEX_FIND_DIRS_CAP.to_vec(), SED_FIND_DEFAULT)
    }

    pub fn timezone(path: &Path) -> Vec<[String; 2]> {
        Self::find(path, 
            REGEX_FIND_FILES.to_vec(), SED_FIND_DEFAULT)
    }

    pub fn find(path: &Path, regex_find: Vec<&str>, regex_sed: &str) -> Vec<[String; 2]> {
        let process_find = Command::new(FIND)
        .current_dir(path.to_str().unwrap())
        .args(regex_find)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("{} {}", ERR_FAILED_EXECUTE_FIND, path.display()));

        let process_sed = Command::new(SED)
        .arg(regex_sed)
        .stdin(Stdio::from(process_find.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("{} {}", ERR_FAILED_EXECUTE_SED, path.display()));

        let output = process_sed
            .wait_with_output()
            .expect(ERR_FAILED_WAIT_SED);        

        let output_string = String::from_utf8_lossy(&output.stdout);
        let mut dirs: Vec<[String; 2]> = Vec::new(); 
        for line in output_string.lines() {
           dirs.push([String::from(line),String::new()]); 
        } 
        dirs
    }
}
