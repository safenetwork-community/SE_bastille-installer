use const_format::{concatcp, formatcp};
use crate::shared::constants::string::SLASH;

// general
pub const DOTS: &str = "..";

// install defaults
// pub const DEFAULT_ARCH: &str = "aarch64";
pub const REGLO_CONSOLEFONT: &str = "eurlatingr";
pub const DEFAULT_INIT: &str = "dinit";
pub const PAHKEHT_AUR_DEFO: &str = "trizen"; 
// pub const REGLO_PAHKEHT_SINISJEHL: &str = "uboot-tools"; 
pub const REGLO_PAHKEHT_FS: &str = "btrfs-progs"; 
pub const REGLO_PAHKEHT_BAZ: &[&str; 2] = &["base-devel", "rsync"]; 
pub const REGLO_PAHKEHT_SXTJEQ: &[&str; 3] = &["cargo", "neovim", "kitty-terminfo"]; 
pub const DEFAULT_OS_ARCH: &str = "armtix";
pub const DEFAULT_OS_BASE: &str = "Artix";
// pub const DEFAULT_OS_FLAVOR: &str = "Bastille OS";
pub const DEFAULT_OS_INIT: &str = formatcp!("{DEFAULT_OS_ARCH}-{DEFAULT_INIT}");
pub const DEFAULT_SHELL: &str = "/bin/bash";
pub const DEFAULT_USERGROUP_USER: &str = DEFAULT_OS_ARCH; 
pub const DEFAULT_USERGROUPS: &str = "wheel,sys,audio,input,video,storage,lp,network,users,power"; 
pub const DEFAULT_USERNAME: &str = DEFAULT_OS_ARCH;

// bootloader types
// pub const BL_COREBOOT: &str = "coreboot";
// pub const BL_U_BOOT: &str = "u-boot";
// pub const BL_UEFI: &str = "uefi";

// filesystem types
pub const TYPE_FS_FAT32: &str = "fat32";
pub const TYPE_FS_BTRFS: &str = "btrfs";

// filesystem sizes
pub const N_BOOT_START: u32 = 32;
pub const N_BOOT_SPACE: u32 = 1024;

// shared texts
pub const TXT_PARTPROBE: &str = concatcp!("Partprobe", DOTS);

// prepare texts
pub const TXT_CREATE_VOLS_MAIN: &str = concatcp!("Create volumes", DOTS);
pub const TXT_CREATE_VOLS_SUB: &str = concatcp!("Create subvolumes", DOTS);
pub const TXT_DD_FIRST_MBS: &str = concatcp!("Clearing first 32mb of drive", DOTS);
pub const TXT_RM_PARTITIONS: &str = concatcp!("Prepare drive partitions", DOTS);


// install os texts
pub const TXT_DOWNLOAD_OS: &str = formatcp!("Downloading latest version of {DEFAULT_OS_BASE}{}", DOTS); 
pub const TXT_EXTRACTING_OS: &str = formatcp!("Extracting {DEFAULT_OS_BASE}{}", DOTS);

// install bootloader texts
pub const TXT_SINISJEHL_INSTALL: &str = concatcp!("Installing bootloader", DOTS);

// setup os texts
pub const TXT_EDITOR: &str = concatcp!("Installing base editor", DOTS);
pub const TXT_FS_INSTALL: &str = concatcp!("Installing btrfs package", DOTS);
pub const TXT_PACKAGES_EQSTALX_AUR: &str = concatcp!("Installing AUR packages", DOTS);
pub const TXT_PACKAGES_EQSTALX_SIPLOJ: &str = concatcp!("Installing OS packages", DOTS);
pub const TXT_PACKAGES_EQSTALX_BAZ: &str = concatcp!("Installing base packages", DOTS);
pub const TXT_PACKAGES_UPDATE: &str = concatcp!("Updating packages", DOTS);
pub const TXT_OTONOMI: &str = concatcp!("Installing base server app", DOTS);
pub const TXT_SETTINGS_SYSTEM: &str = concatcp!("Setting up system settings", DOTS);
pub const TXT_FSTAB: &str = concatcp!("Generating fstab", DOTS);
pub const TXT_USERS: &str = concatcp!("Setting up users", DOTS);



// cleanup texts 
pub const TXT_CLEANUP_INSTALL: &str = concatcp!("Removing unnecessary files", DOTS);
pub const TXT_UMOUNT_DIRS: &str = concatcp!("Unmount directories", DOTS);

// filenames
pub const FILE_QEMU_STATIC: &str = "qemu-aarch64-static";

// main mount directory
pub const LOC_INSTALL:  &str = "/var/tmp/eqstalxr-bastij";

// 
pub const SIN: &str = "boot";
pub const RAS: &str = "root";

// end dirs
pub const LOC_SIN: &str = concatcp!(SLASH, SIN);
pub const LOC_UT: &str = "/home";
pub const LOC_RAS: &str = concatcp!(SLASH, RAS);
pub const LOC_MNT: &str = "/mnt";
pub const ELD_PACMAN_D: &str = "/pacman.d";

// end locations files
pub const ELF_BASHRC: &str = "/.bashrc";

// partition numbers
pub const PART_SIN: u32 = 1;
pub const PART_FUT: u32 = 2;

// volume dirs
pub const LOC_HG_FOQ:  &str = concatcp!(LOC_INSTALL, LOC_MNT);
pub const LOC_HG_SIN: &str = concatcp!(LOC_HG_FOQ, LOC_SIN);
pub const LOC_HG_UT: &str = concatcp!(LOC_HG_FOQ, LOC_UT);

// subvolume names
pub const NAME_SV_FOQ: &str ="@";
pub const NAME_SV_UT: &str = "@home";

// subvolume dirs
pub const ELD_SV_FOQ: &str = concatcp!(SLASH, NAME_SV_FOQ);
pub const ELD_SV_UT: &str = concatcp!(SLASH, NAME_SV_UT);

// fstab
pub const LABEL_SIN: &str = "BASTIJ_SIN";
pub const LABEL_FUT: &str = "BASTIJ_FUT";

// installation directories
pub const DIR_USR_HOST_HOME:  &str = "/home/bas";
pub const DIR_USR_BIN: &str = "/usr/bin";

// host file locations
pub const DIR_FILES: &str = "./files";
pub const LOC_FILES_BOOT: &str = concatcp!(DIR_FILES, LOC_SIN);
pub const LOC_FILES_MIWRAR: &str = concatcp!(DIR_FILES, "/pacman.d");

// installation locations
pub const LOC_BINFMT_AARCH64: &str = "/usr/lib/binfmt.d/qemu-aarch64-static.conf";
pub const LOC_BINFMT_REGISTER: &str = "/proc/sys/fs/binfmt_misc/register";
pub const LOC_DB_LOCK_PACMAN: &str = "/var/lib/pacman/db.lck";
pub const LOC_DEFAULT_BINFMT_ARCH: &str = "/proc/sys/fs/binfmt_misc/qemu-aarch64";
pub const LOC_HOSTNAME: &str = "/etc/hostname";
pub const LOC_FSTAB: &str = "/etc/fstab";
pub const LOC_BASHRC_USER: &str = concatcp!(DIR_USR_HOST_HOME, ELF_BASHRC);
pub const LOC_BASHRC_ROOT: &str = concatcp!(LOC_RAS, ELF_BASHRC);
pub const LOC_LOCALE_CONF: &str = "/etc/locale.conf";
pub const LOC_LOCALE_GEN: &str = "/etc/locale.gen";
pub const LOC_MKINITCPIO_STS: &str = "/etc/mkinitcpio.d/linux-aarch64-lts.preset";
pub const LOC_NVIM_HOST: &str = "/home/bas/.config/nvim";
pub const LOC_PACMAN_D: &str = concatcp!("/etc", ELD_PACMAN_D);
pub const LOC_PACMAN_CONF: &str = "/etc/pacman.conf";
pub const LOC_QEMU_USER_STATIC: &str = formatcp!("{DIR_USR_BIN}/{FILE_QEMU_STATIC}");
pub const LOC_VAR_TMP: &str = "/var/tmp";
pub const LOC_VCONSOLE_CONF: &str = "/etc/vconsole.conf";

// tmp locations
pub const LOC_TMP_BASHRC: &str = concatcp!(LOC_VAR_TMP, ELF_BASHRC);
pub const LOC_TMP_NVIM: &str = concatcp!(LOC_VAR_TMP, "/nvim");
pub const LOC_TMP_SIN: &str = concatcp!(LOC_VAR_TMP, LOC_SIN);
pub const LOC_TMP_PACMAN_D: &str = concatcp!(LOC_VAR_TMP, ELD_PACMAN_D);

// tmp dirs to delete after use
pub const VAR_TMP_DIRS: [&str; 2] = [
    LOC_TMP_NVIM,
    LOC_TMP_SIN
];

pub const VAR_TMP_FILES: [&str; 1] = [
    LOC_TMP_BASHRC
];


// tmp content locations
pub const LOC_TMP_SIN_C: &str = concatcp!(LOC_TMP_SIN, SLASH);
pub const LOC_TMP_NVIM_C: &str = concatcp!(LOC_TMP_NVIM, SLASH);

// test locations
// pub const LOC_MIRRORLIST: &str = "/etc/pacman.d/mirrorlist";

// file mark locations

pub const MAHRK_PROGREHSJOQ: &[(&str, &str); 8] = &[
    (concatcp!(LOC_HG_FOQ, "/mahrk_imazj_koqstrue"), TXT_EXTRACTING_OS),
    (concatcp!(LOC_HG_FOQ, "/mahrk_imazj_datizje"), TXT_PACKAGES_UPDATE),
    (concatcp!(LOC_HG_FOQ, "/mahrk_pakeht_baz_eqstale"), TXT_PACKAGES_EQSTALX_BAZ),
    (concatcp!(LOC_HG_FOQ, "/mahrk_fs_eqstale"), TXT_FS_INSTALL),
    (concatcp!(LOC_HG_FOQ, "/mahrk_sinisjehl_eqstale"), TXT_SINISJEHL_INSTALL),
    (concatcp!(LOC_HG_FOQ, "/mahrk_pakeht_siploj_eqstale"), TXT_PACKAGES_EQSTALX_SIPLOJ),
    (concatcp!(LOC_HG_FOQ, "/mahrk_pahkeht_aur_eqstale"), TXT_PACKAGES_EQSTALX_AUR),
    (concatcp!(LOC_HG_FOQ, "/mahrk_otonomi_eqstale"), TXT_OTONOMI),
]; 

// Host -> Guest installation directories 
pub const LOC_HG_QEMU_USER_STATIC: &str = formatcp!("{LOC_HG_FOQ}{DIR_USR_BIN}/{FILE_QEMU_STATIC}");
pub const LOC_HG_FSTAB: &str = concatcp!(LOC_HG_FOQ, LOC_FSTAB);
pub const LOC_HG_VAR_TMP: &str = concatcp!(LOC_HG_FOQ, LOC_VAR_TMP);
pub const LOC_HG_SV_FOQ: &str = concatcp!(LOC_HG_FOQ, ELD_SV_FOQ);
pub const LOC_HG_SV_UT: &str = concatcp!(LOC_HG_FOQ, ELD_SV_UT);


// URLs
pub const URL_ARMTIX_DL: &str = "https://armtixlinux.org/images/";

// Mirrorlist ALARM
pub const LISTMIWRAR_ALARM: &str = "mirrorlist-archlinuxarm";

// ALARM packages
pub const PACKAGES_ALARM: &[&str; 1] = &[
    "firmware-raspberrypi",
];


