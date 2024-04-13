use const_format::concatcp;

use crate::shared::constants::char::SLASH;

// general
pub const DOTS: &str = "..";

// install defaults
pub const ARCH:&str = "aarch64";

// filesystem types
pub const TYPE_FS_FAT32: &str = "fat32";
pub const TYPE_FS_BTRFS: &str = "btrfs";

// partition types
pub const PRIMARY: &str = "primary";

// istallation directories
pub const DIR_TMP:  &str = "/var/tmp/eqstalxr-bastij";

// shared texts
pub const TXT_PARTPROBE: &str = concatcp!("Partprobe", DOTS);

// prepare texts
pub const TXT_MKLABEL: &str = concatcp!("Make label", DOTS);
pub const TXT_MKBOOT: &str = concatcp!("Make boot partition", DOTS);
pub const TXT_MKROOT: &str = concatcp!("Make root partition", DOTS);
pub const TXT_MKVFAT: &str = concatcp!("Make vfat", DOTS);
pub const TXT_MKBTRFS: &str = concatcp!("Make btrfs", DOTS);
pub const TXT_MKDIR_MNTS: &str = concatcp!("Create mount dirs", DOTS);
pub const TXT_MKSUBVOL_HOME: &str = concatcp!("Create subvolume home", DOTS);
pub const TXT_MKSUBVOL_ROOT: &str = concatcp!("Create subvolume root", DOTS);
pub const TXT_MNT_BOOT: &str = concatcp!("Mount boot", DOTS);
pub const TXT_MNT_MAINVOL_ROOT: &str = concatcp!("Mount root for subvolumes", DOTS);
pub const TXT_MNT_SUBVOLS: &str = concatcp!("Mount subvolumes", DOTS);
pub const TXT_RM_PARTITIONS: &str = concatcp!("Prepare drive partitions", DOTS);
pub const TXT_UMOUNT_ROOT: &str = concatcp!("Unmount root", DOTS);
pub const TXT_DD_FIRST_MBS: &str = concatcp!("Clearing first 32mb of drive", DOTS);

// install os texts
pub const TXT_LIST_MIRROR: &str = concatcp!("Generate mirrorlist", DOTS);
pub const TXT_EXTRACTING_OS: &str = concatcp!("Extracting", DOTS);
pub const TXT_KEYRINGS: &str = concatcp!("Setting up keyrings", DOTS);
pub const TXT_OVERLAY: &str = concatcp!("Extracting", DOTS);
pub const TXT_PACKAGES: &str = concatcp!("Extracting", DOTS);
pub const TXT_PERMISSIONS: &str = concatcp!("Extracting", DOTS);
pub const TXT_SERVICES_ROOT: &str = concatcp!("Extracting", DOTS);
pub const TXT_SERVICES_USER: &str = concatcp!("Extracting", DOTS);
pub const TXT_SETTINGS_SYSTEM: &str = concatcp!("Extracting", DOTS);
pub const TXT_SUPPORT_BTRFS: &str = concatcp!("Extracting", DOTS);
pub const TXT_USERS: &str = concatcp!("Extracting", DOTS);

// non-fixed install os texts
pub const TXT_DOWNLOAD_OS: &str = "Downloading latest version of"; 

// install bootloader texts
// pub const TXT_BOOTLOADER: &str = concatcp!("Make label", DOTS);

// cleanup texts 
pub const TXT_CLEAN_INSTALL: &str = concatcp!("Make label", DOTS);
pub const TXT_UMOUNT_DIRS: &str = concatcp!("Unmount directories", DOTS);

pub const PART_BOOT: u32 = 1;
pub const PART_ROOT: u32 = 2;

// end dirs
pub const BOOT: &str = "boot";
pub const HOME: &str = "home";
pub const ROOT: &str = "root";

// volume dirs
pub const DIR_BOOT: &str = concatcp!(DIR_TMP, SLASH, BOOT);
pub const DIR_ROOT: &str = concatcp!(DIR_TMP, SLASH, ROOT);

pub const DIR_HOME: &str = concatcp!(DIR_ROOT, SLASH, HOME);

// subvolume dirs
pub const DIR_END_SV_ROOT: &str ="@";
pub const DIR_END_SV_HOME: &str = "@home";

pub const DIR_SV_ROOT: &str = concatcp!(DIR_ROOT, SLASH, DIR_END_SV_ROOT);
pub const DIR_SV_HOME: &str = concatcp!(DIR_ROOT, SLASH, DIR_END_SV_HOME);

pub const SUBVOLS_PART_ROOT: [(&str, &str); 2] = [(DIR_END_SV_ROOT, DIR_ROOT), (DIR_END_SV_HOME, DIR_HOME)];

// install dirs
pub const DIR_PAC_PKG: &str = concatcp!(DIR_ROOT, "/var/cache/pacman/pkg");
pub const DIR_PKG_CACHE: &str = concatcp!(DIR_TMP, "/pkg-cache");


// URLs
pub const URL_ARMTIX_DL: &str = "https://armtixlinux.org/images/";

// filenames
pub const FILE_XZ_ARMTIX: &str = "armtix-dinit-20240303.tar.xz";
