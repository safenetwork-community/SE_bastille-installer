use const_format::{formatcp, str_split};

use crate::shared::constants::char::SPACE;

// General commands
pub const LSBLK: &str = "lsblk";
pub const SH: &str = "sh"; 
pub const SUDO: &str = "sudo";

// General arguments
pub const DEV_NULL: &str = "/dev/null";
pub const ONE_G: &str = "1>";
pub const TWO_GN_ONE: &str = "2>&1";

// Shell arguments
pub const ARG_C: &str = "-c";
pub const EOA: &str = "--";
pub const ARG_SH_C: [&str; 3] = [EOA, SH, ARG_C];

// general sh -c arguments
pub const ASC_QUIET: &str = formatcp!("{ONE_G} {DEV_NULL} {TWO_GN_ONE}");
pub const ASCS_QUIET: [&str; 3] = str_split!(ASC_QUIET, SPACE);
