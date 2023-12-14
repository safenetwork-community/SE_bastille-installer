use std::path::Path;
use std::process::{Command, Stdio};

use crate::app::r#box::Page;

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

pub const LIST_MENU_MAIN: &[(&str, Page)] = &[
    ("Start installation", Page::Username), 
    ("Change keyboard layout", Page::KeymapHost),
    ("Quit", Page::Quit)
];

pub const LIST_MENU_CONFIG: &[(&str, Page)] = &[
    ("enter username", Page::Username),
    ("enter usergroups", Page::Usergroups),
    ("enter fullname", Page::Fullname),
    ("set password of user", Page::PasswordUserSgn),
    ("set password of root", Page::PasswordRootSgn),
    ("select drive", Page::Drive),
    ("select timezone", Page::TimezoneRegion),
    ("enter hostname", Page::Hostname)
];

pub struct ListFromCommand {}

impl ListFromCommand {
    pub fn drives() -> Vec<[String; 2]> {
        let args: [&str; 3] = ["-dn","-o", "NAME"];
        let ls_output = Command::new("lsblk")
        .args(args)
        .output()
        .expect(format!("Failed to execute lsblk {:?}", args).as_str());
 
        let mut list: Vec<[String; 2]> = Vec::new(); 
        let output = String::from_utf8_lossy(&ls_output.stdout);
        for line in output.lines() {
           list.push([String::from(line),String::new()]); 
        } 
        return list
    }

    pub fn keyvars(path: &Path) -> Vec<[String; 2]> {
        Self::find(path, 
            REGEX_FIND_FILES.to_vec(), SED_FIND_FILE_EXTENSIONS)
    }

    pub fn keymap() -> Vec<[String; 2]> {
        Self::find(Path::new(PATH_BKEYMAP), 
            REGEX_FIND_DIRS_ALL.to_vec(), SED_FIND_DEFAULT)
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
        let process_find = Command::new("find")
        .current_dir(path.to_str().unwrap())
        .args(regex_find)
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("Failed to execute find {}", path.display()).as_str());

        let process_sed = Command::new("sed")
        .arg(regex_sed)
        .stdin(Stdio::from(process_find.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("Failed to execute sed {}", path.display()).as_str());

        let output = process_sed
            .wait_with_output()
            .expect("Failed to wait on sed process");        

        let output_string = String::from_utf8_lossy(&output.stdout);
        let mut dirs: Vec<[String; 2]> = Vec::new(); 
        for line in output_string.lines() {
           dirs.push([String::from(line),String::new()]); 
        } 
        dirs
    }
}
