use const_format::formatcp;

// General commands
pub const ARTIX_CHROOT: &str = "artix-chroot";
// pub const BASH: &str = "bash";
pub const BTRFS: &str = "btrfs";
pub const CAT: &str = "cat";
pub const CP: &str = "cp";
pub const DD: &str = "dd";
pub const ECHO: &str = "echo";
pub const FSTABGEN: &str = "fstabgen";
pub const GETENT: &str = "getent";
pub const GIT: &str = "git";
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
pub const RSYNC: &str = "rsync";
pub const SED: &str = "sed";
pub const SH: &str = "sh"; 
pub const SUDO: &str = "sudo";
pub const TAR: &str = "tar";
pub const TEE: &str = "tee";
pub const TOUCH: &str = "touch";
pub const TRIZEN: &str = "trizen";
pub const UMOUNT: &str = "umount";
pub const USERMOD: &str = "usermod";
// pub const WGET: &str = "wget";

// General arguments
/*
*/

// Shell arguments

// general arguments
pub const ARG_A: &str = "-a";
pub const ARG_C: &str = "-c";
pub const ARG_D: &str = "-d";
pub const ARG_DN: &str = "-dn";
pub const ARG_F: &str = "-f";
pub const ARG_I: &str = "-i";
pub const ARG_L: &str = "-l";
pub const ARG_LN: &str = "-ln";
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
// pub const ARG_Q: &str = "-q";
pub const ARG_RF: &str = "-rf";
pub const ARG_S: &str = "-s";
pub const ARG_T: &str = "-t";
pub const ARG_U: &str = "-u";
pub const ARG_XF: &str = "-xf";
pub const ARGS_C: &str = "-C";
pub const ARGS_G: &str = "-G";
pub const ARGS_L: &str = "-L";
pub const ARGS_S: &str = "-S";
pub const ARGS_U: &str = "-U";
pub const ARGS_SYYU: &str = "-Syyu";

// long arguments
pub const ARL_DIR: &str = "--directory";
pub const ARL_EXCLUDE: &str = "--exclude";
pub const ARL_FILTER: &str = "--filter";
pub const ARL_GLOBAL: &str = "--global";
pub const ARL_GROUP_IS: &str = "--group=";
pub const ARL_NOCONFIRM: &str = "--noconfirm";
pub const ARL_OWNER_IS: &str = "--owner=";

//
// command specific arguments
pub const ACS_BS_1M: &str = "bs=1M";
pub const ACS_C_PERCENT: &str = "100%";
pub const ACS_CR: &str = "cr";
pub const ACS_COUNT_32: &str = "count=32";
pub const ACS_DEV_NULL: &str = "/dev/null";
pub const ACS_DISK: &str = "disk";
pub const ACS_GPT: &str = "gpt";
pub const ACS_GROUP: &str = "group";
pub const ACS_IF_DEV_ZERO: &str = "if=/dev/zero";
pub const ACS_MAIN_VOL_COMPRESS: &str = "compress=zstd";
pub const ACS_MIB: &str = "MiB";
pub const ACS_MOUNTPOINTS: &str = "mountpoints";
pub const ACS_OPTIMAL: &str = "optimal";
pub const ACS_PART: &str = "part";
pub const ACS_PASSWD: &str = "passwd";
pub const ACS_STATUS_NONE: &str = "status=none";
pub const ACS_SINGLE: &str = "single";
pub const ACS_SU: &str = "su";
pub const ACS_SUB_VOL_COMPRESS: &str = formatcp!("{ACS_MAIN_VOL_COMPRESS},subvol=");
pub const ACS_UNIT: &str = "unit";
