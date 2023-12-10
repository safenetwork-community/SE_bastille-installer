use const_format::concatcp;

// General
pub const OS: &str = "SE Bastille installer";
pub const VERSION: &str = "0.1.0";

pub const TITRFOQ: &str = concatcp!("Èstalxr d sistêmbêstur d Bastij", " ", VERSION);
pub const TITLE: &str = concatcp!(OS, " ", VERSION);

// General empty string
pub const EMPTY: &str = "";

// General box options
pub const CLEAR: &str = "clear";

// General box labels
pub const LABEL_BACK: &str = "Back";
pub const LABEL_QUIT: &str = "Quit";

// Dimensions Menu box
pub const HEIGHT_BOX_MENU: u32 = 20;
pub const WIDTH_BOX_MENU: u32 = 50;
pub const HEIGHT_LIST_MENU: u32 = 15;

// Dimensions password box
pub const HEIGHT_BOX_PASSWORD: u32 = 11;
pub const WIDTH_BOX_PASSWORD: u32 = 90;

// Dimensions input box
pub const HEIGHT_BOX_INPUT: u32 = 11;
pub const WIDTH_BOX_INPUT: u32 = 90;

// Input defaults
pub const DEFAULT_FULLNAME: &str = "Useur Bastille";
pub const DEFAULT_HOSTNAME: &str = "reseau-sur";
pub const DEFAULT_USERNAME: &str = "bas";

// Unexpected error messages
pub const EXP_DBOX: &str = "Could not display dialog box.";

// general regex
pub const REGEX_HOSTNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];
pub const REGEX_USERNAME: [&str; 3] = [r"[A-Z]", r"[0-9]", r"[!@#\$%^\&*()_+/\\]"];

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

// Errors
pub const ERROR_EMPTY_MENU: &str = "\nExpected at least 20 tokens for --men, have 4.\nUse --help to list options.\n\n\n";

