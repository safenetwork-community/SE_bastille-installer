use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app::commands::action::CommandAction;

use crate::shared::constants::install::*;
use crate::shared::constants::char::SLASH;

pub struct BuilderListCommand<'a> {
    device: &'a str,
    drive: &'a Path,
    key_pub: &'a str,
    keymap: &'a str,
    locale: &'a str,
    name_full: &'a str,
    name_host: &'a str,
    name_user: &'a str,
    partition_boot: PathBuf,
    partition_root: PathBuf,
    password_root: &'a str,
    password_user: &'a str,
    region_timezone: &'a str,
    zone_timezone: &'a str
}

impl BuilderListCommand<'_> {
    pub fn new<'a>(device: &'a str, name_user: &'a str, name_full: &'a str, 
        password_user: &'a str, password_root: &'a str, key_pub: &'a str, 
        drive: &'a Path,  keymap: &'a str, locale: &'a str, 
        region_timezone: &'a str, zone_timezone: &'a str, 
        name_host: &'a str) -> BuilderListCommand<'a> {
        BuilderListCommand {
            device,
            drive,
            key_pub,
            keymap,
            locale, 
            name_full,
            name_host,
            name_user,
            partition_boot: Self::get_partition(drive, PART_BOOT), 
            partition_root: Self::get_partition(drive, PART_ROOT), 
            password_root,
            password_user,
            region_timezone,
            zone_timezone
        }
    } 

    pub fn prepare(&self) -> Vec<(String, Option<Command>)> {
        match self.device {
            "rpi4" => vec![
                (String::from(TXT_RM_PARTITIONS), CommandAction::remove_partitions(self.drive)),
                (String::from(TXT_DD_FIRST_MBS), CommandAction::dd_first_mbs(self.drive)),
                (String::from(TXT_MKLABEL), CommandAction::make_label(self.drive)),
                (String::from(TXT_MKBOOT), CommandAction::make_boot_partition(self.drive, TYPE_FS_FAT32)),
                (String::from(TXT_MKROOT), CommandAction::make_root_partition(self.drive, TYPE_FS_BTRFS, PART_BOOT)),
                (String::from(TXT_PARTPROBE), CommandAction::partprobe(self.drive)),
                (String::from(TXT_MKVFAT), CommandAction::mkfs_vfat(self.drive, PART_BOOT)),
                (String::from(TXT_MKBTRFS), CommandAction::mkfs_btrfs(self.drive, PART_ROOT)),
                (String::from(TXT_MKDIR_MNTS), CommandAction::make_dirs(&[DIR_HG_ROOT, DIR_HG_BOOT])),
                (String::from(TXT_MNT_MAINVOL_ROOT), CommandAction::mount_mainvol(&self.partition_root, DIR_HG_ROOT)),
                (String::from(TXT_MKSUBVOL_ROOT), CommandAction::make_subvol(Path::new(DIR_SV_ROOT))),
                (String::from(TXT_MKSUBVOL_HOME), CommandAction::make_subvol(Path::new(DIR_SV_HOME))),
                (String::from(TXT_UMOUNT_ROOT), CommandAction::umount(&self.partition_root)),
                (String::from(TXT_MNT_SUBVOLS), CommandAction::mount_subvols(&self.partition_root, &SUBVOLS_PART_ROOT)),
                (String::from(TXT_MNT_BOOT), CommandAction::mount(&self.partition_boot, DIR_HG_BOOT)),
            ],
            _ => vec![],
        }
    }
 
    pub fn install_os(&self) -> Vec<(String, Option<Command>)> {
        match self.device {
            "rpi4" => vec![ 
                (String::from(TXT_DOWNLOAD_OS), CommandAction::wget(DIR_MNT, format!("{URL_ARMTIX_DL}{FILE_XZ_ARMTIX}").as_str())),
                (String::from(TXT_EXTRACTING_OS), CommandAction::extract_rootfs(format!("{DIR_MNT}{SLASH}{FILE_XZ_ARMTIX}").as_str(), DIR_HG_ROOT)),
                (String::from(TXT_BR_ARCH_GAP), CommandAction::bridge_arch_gap())
            ],
            _ => vec![],
        }
    }

    pub fn install_bootloader(&self) -> Vec<(String, Option<Command>)> { 
        match self.device {
            "rpi4" => vec![
                (String::from(TXT_INSTALL_BOOTLOADER_BUILDER), CommandAction::eqstalx_packages(DEFAULT_BOOTLOADER)),
                (String::from(TXT_INSTALL_BOOTLOADER), CommandAction::eqstalx_bootloader()),
                (String::from(TXT_FSTAB), CommandAction::zjenx_fstab())
            ],
            _ => vec![],
        }
    }

    pub fn setup_os(&self) -> Vec<(String, Option<Command>)> {
        match self.device {
            "rpi4" => vec![ 
                (String::from(TXT_PACKAGES_UPDATE), CommandAction::update_packages()),
                (String::from(TXT_PACKAGES_INSTALL), CommandAction::eqstalx_packages(DEFAULT_PACKAGES)),
                (String::from(TXT_PACKAGE_FS), CommandAction::eqstalx_fs()),
                (String::from(TXT_USERS), CommandAction::set_users(self.name_user, self.name_full, 
                self.password_user, self.password_root, self.key_pub)),
                (String::from(TXT_REZOSUR), CommandAction::azjx_rezosur()),
                (String::from(TXT_EDITOR), CommandAction::azjx_editor()),
                (String::from(TXT_SETTINGS_SYSTEM), CommandAction::set_settings_system(
                self.region_timezone, self.zone_timezone, self.locale, self.keymap, self.name_host)),
            ],
            _ => vec![],
        }
    }

    pub fn cleanup(&self) -> Vec<(String, Option<Command>)> { 
        match self.device {
            "rpi4" => vec![
                (String::from(TXT_CLEAN_INSTALL), CommandAction::clean_up_install(DEFAULT_ARCH)),
                (String::from(TXT_UMOUNT_DIRS), CommandAction::umount_dirs(&[DIR_HG_BOOT, DIR_HG_HOME, DIR_HG_ROOT])),
                (String::from(TXT_PARTPROBE), CommandAction::partprobe(self.drive))
            ],
            _ => vec![]
        }
    }

    pub fn build(self) -> Vec<(String, Option<Command>)> {
        let mut builder = self.prepare();
        builder.extend(self.install_os());
        builder.extend(self.install_bootloader());
        builder.extend(self.setup_os());
        builder.extend(self.cleanup());
        builder
    }

    pub fn build_2(self) -> Vec<(String, Option<Command>)> {
        let mut builder = self.prepare();
        builder.extend(self.install_os());
        // builder.extend(self.install_bootloader());
        builder.extend(self.setup_os());
        builder.extend(self.cleanup());
        builder
    }


    fn get_partition(drive: &Path, partition: u32) -> PathBuf {
        let mut drive: PathBuf = drive.to_path_buf();
        match drive.clone().file_name() {
            Some(os_str_drive) => {
                match os_str_drive.to_str() {
                    Some(str_drive) => drive.set_file_name(format!("{str_drive}{partition}")),
                    None => panic!("Path to drive contains invalid characters."),
                }
            }, 
            None => panic!("Drive filename equals two dots."),
        }
        drive
    }
}
