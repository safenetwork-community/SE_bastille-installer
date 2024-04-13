use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app::commands::action::CommandAction;

use crate::shared::constants::install::*;
use crate::shared::constants::char::SLASH;

pub struct BuilderListCommand<'a> {
    device: &'a str,
    drive: &'a Path,
    name_os: &'a str,
    partition_boot: PathBuf,
    partition_root: PathBuf,
    password_user: &'a str
}

impl BuilderListCommand<'_> {
    pub fn new<'a>(device: &'a str, drive: &'a Path, name_os: &'a str, password_user: &'a str) -> BuilderListCommand<'a> {
        BuilderListCommand {
            device,
            drive,
            name_os,
            partition_boot: Self::get_partition(drive, PART_BOOT), 
            partition_root: Self::get_partition(drive, PART_ROOT), 
            password_user        
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
                (String::from(TXT_MKDIR_MNTS), CommandAction::make_dirs(&[DIR_ROOT, DIR_BOOT])),
                (String::from(TXT_MNT_MAINVOL_ROOT), CommandAction::mount_mainvol(&self.partition_root, DIR_ROOT)),
                (String::from(TXT_MKSUBVOL_ROOT), CommandAction::make_subvol(Path::new(DIR_SV_ROOT))),
                (String::from(TXT_MKSUBVOL_HOME), CommandAction::make_subvol(Path::new(DIR_SV_HOME))),
                (String::from(TXT_UMOUNT_ROOT), CommandAction::umount(&self.partition_root)),
                (String::from(TXT_MNT_SUBVOLS), CommandAction::mount_subvols(&self.partition_root, &SUBVOLS_PART_ROOT)),
                (String::from(TXT_MNT_BOOT), CommandAction::mount(&self.partition_boot, DIR_BOOT)),
            ],
            _ => vec![],
        }
    }
 
    pub fn install_os(&self) -> Vec<(String, Option<Command>)> {
        match self.device {
            "rpi4" => vec![ 
                (format!("{TXT_DOWNLOAD_OS} {}{DOTS}", self.name_os), 
                    CommandAction::wget(DIR_TMP, format!("{URL_ARMTIX_DL}{FILE_XZ_ARMTIX}").as_str())),
                (String::from(TXT_EXTRACTING_OS), CommandAction::extract_rootfs(format!("{DIR_TMP}{SLASH}{FILE_XZ_ARMTIX}").as_str(), DIR_ROOT)),
                (String::from(TXT_KEYRINGS), CommandAction::setup_keyrings(DIR_ROOT)),
                (String::from(TXT_LIST_MIRROR), CommandAction::set_list_mirror(DIR_ROOT)),
                (String::from(TXT_PACKAGES), CommandAction::install_packages(DIR_ROOT)),
                (String::from(TXT_SERVICES_ROOT), CommandAction::enable_services_root(DIR_ROOT)),
                (String::from(TXT_OVERLAY), CommandAction::apply_overlay()),
                (String::from(TXT_USERS), CommandAction::add_users(DIR_ROOT, self.password_user)),
                (String::from(TXT_SERVICES_USER), CommandAction::enable_services_user()),
                (String::from(TXT_SETTINGS_SYSTEM), CommandAction::set_settings_system()),
                (String::from(TXT_PERMISSIONS), CommandAction::set_settings_system()),
                (String::from(TXT_SUPPORT_BTRFS), CommandAction::add_support_btrfs())
            ],
            _ => vec![],
        }
    }

/*
    pub fn install_bootloader(&self) -> Vec<(String, Option<Command>)> { 
        match self.device {
            "rpi4" => vec![
                (String::from(TXT_DOWNLOAD_BOOTLOADER), CommandAction::git_clone(DIR_TMP, format!("{GIT_BOOTLOADER}").as_str())),
                (String::from(TXT_BUILD_BOOTLOADER), CommandAction::build_bootloader())
                (String::from(TXT_INSTALL_BOOTLOADER), CommandAction::install_bootloader())
            ],
            _ => vec![],
        }
    }
*/
    pub fn cleanup(&self) -> Vec<(String, Option<Command>)> { 
        match self.device {
            "rpi4" => vec![
                (String::from(TXT_CLEAN_INSTALL), CommandAction::clean_up_install(ARCH)),
                (String::from(TXT_UMOUNT_DIRS), CommandAction::umount_dirs(&[DIR_HOME, DIR_ROOT, DIR_BOOT])),
                (String::from(TXT_PARTPROBE), CommandAction::partprobe(self.drive))
            ],
            _ => vec![]
        }
    }

    pub fn build(self) -> Vec<(String, Option<Command>)> {
        let mut builder = Vec::new(); // self.prepare();
        builder.extend(self.install_os());
        // builder.extend(self.install_bootloader());
        builder.extend(self.cleanup());
        builder
    }

    pub fn build_2(self) -> Vec<(String, Option<Command>)> {
        let mut builder = self.prepare();
        builder.extend(self.install_os());
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
