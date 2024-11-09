use const_format::{concatcp, formatcp};

// general
pub const DOTS: &str = "..";

// install defaults
// pub const DEFAULT_ARCH: &str = "aarch64";
// pub const DEFAULT_BOOTLOADER: &str = "grub";
pub const DEFAULT_CONSOLEFONT: &str = "eurlatingr";
pub const DEFAULT_EDITOR: &str = "lunarvim";
pub const DEFAULT_INIT: &str = "dinit";
// pub const DEFAULT_PACKAGE_BOOTLOADER_GRUB: &str = "grub os-prober efibootmgr"; 
pub const DEFAULT_PACKAGE_FS: &str = "btrfs-progs"; 
pub const DEFAULT_PACKAGES: &str = "cargo neovim kitty-terminfo"; 
pub const DEFAULT_OS_BASE: &str = "Artix";
pub const DEFAULT_OS_FLAVOR: &str = "Bastille OS";
pub const DEFAULT_SHELL: &str = "/bin/bash";
pub const DEFAULT_URL_EDITOR: &str = "https://raw.githubusercontent.com/LunarVim/LunarVim/release-1.3/neovim-0.9/utils/installer/install.sh";
pub const DEFAULT_USERGROUP_USER: &str = "armtix"; 
pub const DEFAULT_USERGROUPS: &str = "wheel,sys,audio,input,video,storage,lp,network,users,power"; 
pub const DEFAULT_USERNAME: &str = "armtix";

// bootloader types
// pub const BL_COREBOOT: &str = "coreboot";
// pub const BL_U_BOOT: &str = "u-boot";
// pub const BL_UEFI: &str = "uefi";

// filesystem types
pub const TYPE_FS_FAT32: &str = "fat32";
pub const TYPE_FS_BTRFS: &str = "btrfs";

// partition types
pub const PRIMARY: &str = "primary";

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
// pub const TXT_INSTALL_BOOTLOADER: &str = concatcp!("Installing bootloader", DOTS);

// setup os texts
pub const TXT_EDITOR: &str = formatcp!("Installing base editor {DEFAULT_EDITOR}{}", DOTS);
// pub const TXT_KEYRINGS: &str = concatcp!("Setting up keyrings", DOTS);
pub const TXT_PACKAGE_FS: &str = concatcp!("Installing btrfs package", DOTS);
pub const TXT_PACKAGES_INSTALL: &str = concatcp!("Installing packages", DOTS);
pub const TXT_PACKAGES_UPDATE: &str = concatcp!("Updating packages", DOTS);
pub const TXT_REZOSUR: &str = formatcp!("Installing base server app {DEFAULT_OS_FLAVOR}{}", DOTS);
pub const TXT_SETTINGS_SYSTEM: &str = concatcp!("Setting up system settings", DOTS);
pub const TXT_FSTAB: &str = concatcp!("Generating fstab", DOTS);
pub const TXT_USERS: &str = concatcp!("Setting up users", DOTS);



// cleanup texts 
pub const TXT_CLEAN_INSTALL: &str = concatcp!("Make label", DOTS);
pub const TXT_UMOUNT_DIRS: &str = concatcp!("Unmount directories", DOTS);

// URLs
pub const URL_ARMTIX_DL: &str = "https://armtixlinux.org/images/";

// filenames
pub const FILE_XZ_ARMTIX: &str = "armtix-dinit-20240831.tar.xz";
pub const FILE_QEMU_STATIC: &str = "qemu-aarch64-static";
pub const FILE_MAHRK_IMAZJ_KOQSTRUE: &str = "mahrk_imazj_koqstrue";
pub const FILE_MAHRK_IMAZJ_DATIZJE: &str = "mahrk_imazj_datizje";

// main mount directory
pub const DIR_MNT:  &str = "/var/tmp/eqstalxr-bastij";

// end dirs
pub const BOOT: &str = "boot";
pub const HOME: &str = "home";
pub const ROOT: &str = "root";

// partition numbers
pub const PART_BOOT: u32 = 1;
pub const PART_ROOT: u32 = 2;

// volume dirs
pub const DIR_HG_BOOT: &str = formatcp!("{DIR_MNT}/{ROOT}/{BOOT}");
pub const DIR_HG_ROOT: &str = formatcp!("{DIR_MNT}/{ROOT}");
pub const DIR_HG_HOME: &str = formatcp!("{DIR_HG_ROOT}/{HOME}");

// subvolume dirs
pub const DIR_NAME_SV_ROOT: &str ="@";
pub const DIR_NAME_SV_HOME: &str = "@home";

pub const DIR_SV_ROOT: &str = formatcp!("{DIR_HG_ROOT}/{DIR_NAME_SV_ROOT}");
pub const DIR_SV_HOME: &str = formatcp!("{DIR_HG_ROOT}/{DIR_NAME_SV_HOME}");

// fstab
pub const LABEL_BOOT: &str = "BASTIJ_SIN";
pub const LABEL_ROOT_AND_HOME: &str = "BASTIJ_FUT";

// installation directories
pub const DIR_HOME:  &str = "/home/bas";
// pub const DIR_SYS_BLOCK: &str = "/sys/block";
pub const DIR_USR_BIN: &str = "/usr/bin";

// istallation locations
pub const LOC_BINFMT_AARCH64: &str = "/usr/lib/binfmt.d/qemu-aarch64-static.conf";
pub const LOC_BINFMT_REGISTER: &str = "/proc/sys/fs/binfmt_misc/register";
pub const LOC_DEFAULT_BINFMT_ARCH: &str = "/proc/sys/fs/binfmt_misc/qemu-aarch64";
pub const LOC_HOSTNAME: &str = "/etc/hostname";
pub const LOC_FSTAB: &str = "/etc/fstab";
pub const LOC_PROFILE: &str = formatcp!("{DIR_HOME}/.profile");
pub const LOC_LOCALE_CONF: &str = "/etc/locale.conf";
pub const LOC_LOCALE_GEN: &str = "/etc/locale.gen";
pub const LOC_MKINITCPIO_STS: &str = "/etc/mkinitcpio.d/linux-aarch64.preset";
pub const LOC_QEMU_USER_STATIC: &str = formatcp!("{DIR_USR_BIN}/{FILE_QEMU_STATIC}");
pub const LOC_VCONSOLE_CONF: &str = "/etc/vconsole.conf";
pub const LOC_DB_LOCK_PACMAN: &str = "/var/lib/pacman/db.lck";

// Host -> Guest installation directories 
pub const LOC_HG_QEMU_USER_STATIC: &str = formatcp!("{DIR_HG_ROOT}{DIR_USR_BIN}/{FILE_QEMU_STATIC}");
pub const LOC_HG_MAHRK_IMAZJ_KOQSTRUE: &str = formatcp!("{DIR_HG_ROOT}/{FILE_MAHRK_IMAZJ_KOQSTRUE}");
pub const LOC_HG_MAHRK_IMAZJ_DATIZJE: &str = formatcp!("{DIR_HG_ROOT}/{FILE_MAHRK_IMAZJ_DATIZJE}");
pub const LOC_HG_MAHRK_IMAZJ_FINI: &str = LOC_HG_MAHRK_IMAZJ_KOQSTRUE;
pub const LOC_HG_FSTAB: &str = formatcp!("{DIR_HG_ROOT}{LOC_FSTAB}");
pub const LOC_FILE_XZ_ARMTIX: &str = formatcp!("{DIR_MNT}/{FILE_XZ_ARMTIX}");
