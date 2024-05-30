use const_format::formatcp;

// General commands
pub const LSBLK: &str = "lsblk";
pub const RM: &str = "rm";
pub const SH: &str = "sh"; 
pub const SUDO: &str = "sudo";

// General arguments
/*
pub const DEV_NULL: &str = "/dev/null";
pub const ONE_G: &str = "1>";
pub const TWO_GN_ONE: &str = "2>&1";
*/

// Shell arguments
pub const ARG_C: &str = "-c";
pub const EOA: &str = "--";
pub const ARG_SH_C: [&str; 3] = [EOA, SH, ARG_C];

// general sh -c arguments
pub const ASC_QUIET: &str = formatcp!("");
//pub const ASC_QUIET: &str = formatcp!("{ONE_G} {DEV_NULL} {TWO_GN_ONE}");
