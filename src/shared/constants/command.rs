use const_format::formatcp;

// General commands
pub const ARTIX_CHROOT: &str = "artix-chroot";
pub const AWK: &str = "awk";
pub const BTRFS: &str = "btrfs";
pub const CAT: &str = "cat";
pub const CURL: &str = "curl";
pub const DD: &str = "dd";
pub const ECHO: &str = "echo";
pub const FSTABGEN: &str = "fstabgen";
pub const GETENT: &str = "getent";
pub const GROUPMOD: &str = "groupmod";
pub const INSTALL: &str = "install";
pub const LN: &str = "ln";
pub const LOCALE_GEN: &str = "locale-gen";
pub const LSBLK: &str = "lsblk";
pub const MKDIR: &str = "mkdir";
pub const MKFS_BTRFS: &str = "mkfs.btrfs";
pub const MKFS_VFAT: &str = "mkfs.vfat";
pub const MKLABEL: &str = "mklabel";
pub const MKPART: &str = "mkpart";
pub const MOUNT: &str = "mount";
pub const PACMAN: &str = "pacman";
pub const PARTED: &str = "parted";
pub const PARTPROBE: &str = "partprobe";
pub const RM: &str = "rm";
pub const SED: &str = "sed";
pub const SH: &str = "sh"; 
pub const SUDO: &str = "sudo";
pub const TAR: &str = "tar";
pub const TEE: &str = "tee";
pub const TOUCH: &str = "touch";
pub const UMOUNT: &str = "umount";
pub const USERMOD: &str = "usermod";
pub const WGET: &str = "wget";

// General arguments
/*
*/

// Shell arguments
pub const ARG_C: &str = "-c";
pub const EOA: &str = "--";


// general arguments
pub const ARG_A: &str = "-a";
pub const ARG_D: &str = "-d";
pub const ARG_E: &str = "-e";
pub const ARG_F: &str = "-f";
pub const ARG_I: &str = "-i";
pub const ARG_L: &str = "-l";
pub const ARG_LP: &str = "-lp";
pub const ARG_M: &str = "-m";
pub const ARG_MOD600: &str = "-m600";
pub const ARG_MOD644: &str = "-m644";
pub const ARG_MOD700: &str = "-m700";
pub const ARG_MOD755: &str = "-m755";
pub const ARG_N: &str = "-n";
pub const ARG_NO: &str = "-no";
pub const ARG_O: &str = "-o";
pub const ARG_P: &str = "-p";
pub const ARG_Q: &str = "-q";
pub const ARG_RF: &str = "-rf";
pub const ARG_S: &str = "-s";
pub const ARG_XF: &str = "-xf";
pub const ARGS_C: &str = "-C";
pub const ARGS_G: &str = "-G";
pub const ARGS_L: &str = "-L";
pub const ARGS_S: &str = "-S";
pub const ARGS_U: &str = "-U";
pub const ARGS_SYYU: &str = "-Syyu";

// long arguments
pub const ARL_DIR: &str = "--directory";
pub const ARL_FILTER: &str = "--filter";
pub const ARL_GROUP_IS: &str = "--group=";
pub const ARL_NOCONFIRM: &str = "--noconfirm";
pub const ARL_OWNER_IS: &str = "--owner=";

// command specific arguments
pub const ACS_1MIB: &str = "1MiB";
pub const ACS_CR: &str = "cr";
pub const ACS_MOUNTPOINTS: &str = "mountpoints";
pub const ACS_PRINT: &str = "print";
pub const ACS_PRINT_C1_BW_SPACE: &str = "/^ / {print $1}";
pub const ACS_SU: &str = "su";
pub const ACS_BOOT_SPACE: &str = "512M";
pub const BS_1M: &str = "bs=1M";
pub const COUNT_32: &str = "count=32";
// pub const DEV_NULL: &str = "/dev/null";
pub const EOF: &str = "EOF";
pub const GPT: &str = "gpt";
pub const C_PERCENT: &str = "100%";
pub const IF_DEV_ZERO: &str = "if=/dev/zero";
pub const FILE_LITERAL: &str = "<<";
pub const MAIN_VOL_COMPRESS: &str = "compress=zstd";
// pub const ONE_G: &str = "1>";
pub const OPTIMAL: &str = "optimal";
pub const SINGLE: &str = "single";
pub const STATUS_NONE: &str = "status=none";
pub const SUB_VOL_COMPRESS: &str = formatcp!("{MAIN_VOL_COMPRESS},subvol=");
// pub const TWO_GN_ONE: &str = "2>&1";
