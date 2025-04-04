use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};


use crate::app::commands::run::*;

use crate::shared::constants::install::*;

pub struct RcInstall {
    path_os: Rc<RefCell<PathBuf>>,
}

impl RcInstall {
    pub fn new() -> RcInstall {
        RcInstall {
            path_os: Rc::new(RefCell::new(PathBuf::new())),
        }
    }
}

pub struct ListCommand {
    device: String,
    drive: PathBuf,
    key_pub: String,
    keymap: String,
    locale: String,
    name_full: String,
    name_host: String,
    name_user: String,
    partition_boot: PathBuf,
    partition_root: PathBuf,
    password_root: String,
    password_user: String,
    region_timezone: String,
    zone_timezone: String
}

impl ListCommand {
    pub fn new(device: &str, name_user: &str, name_full: &str, 
        password_user: &str, password_root: &str, 
        key_pub: &str, drive: &Path, 
        keymap: &str, locale: &str, 
        region_timezone: &str, zone_timezone: &str, 
        name_host: &str) -> ListCommand {
        ListCommand {
            device: String::from(device),
            drive: drive.into(),
            key_pub: String::from(key_pub),
            keymap: String::from(keymap),
            locale: String::from(locale),
            name_full: String::from(name_full),
            name_host: String::from(name_host),
            name_user: String::from(name_user),
            partition_boot: PathBuf::from(format!("{}{PART_SIN}", drive.display())), 
            partition_root: PathBuf::from(format!("{}{PART_FUT}", drive.display())), 
            password_root: String::from(password_root),
            password_user: String::from(password_user),
            region_timezone: String::from(region_timezone),
            zone_timezone: String::from(zone_timezone) 
        }
    } 

    pub fn get_dydeh_command(&self) -> Vec<(String, Vec<Box<dyn CommandRun>>)> {
        let rc_install = RcInstall::new();

        match self.device.as_str() {
            "test" => vec![
                (String::from(TXT_RM_PARTITIONS), vec![ 
                    Box::new(UmountDrive::new(&self.drive)), 
                    Box::new(RemovePartitionsDrive::new(&self.drive))
                ]),
                (String::from(TXT_DD_FIRST_MBS), vec![
                    Box::new(DdFirstMbs::new(&self.drive))
                ]),
                (String::from(TXT_CREATE_VOLS_MAIN), vec![
                    Box::new(MakeLabel::new(&self.drive)),
                    Box::new(MakePartitionBoot::new(&self.drive, TYPE_FS_FAT32)),
                    Box::new(MakePartitionRoot::new(&self.drive, TYPE_FS_BTRFS)), 
                    Box::new(Partprobe::new(&self.drive)),
                    Box::new(MkfsVfat::new(&self.drive, PART_SIN)),
                    Box::new(MkfsBtrfs::new(&self.drive, PART_FUT))
                ]),
                (String::from(TXT_CREATE_VOLS_SUB), vec![
                    Box::new(MakeDir::new(Path::new(LOC_HG_FOQ))),
                    Box::new(MountVolumeMain::new(&self.partition_root, Path::new(LOC_HG_FOQ))),
                    Box::new(MkfsBtrfsSub::new(Path::new(LOC_HG_SV_FOQ))),
                    Box::new(MkfsBtrfsSub::new(Path::new(LOC_HG_SV_UT))),
                    Box::new(UmountPartition::syl(&self.partition_root)),
                    Box::new(MountVolumeSub::new(NAME_SV_FOQ, &self.partition_root, Path::new(LOC_HG_FOQ))),
                    Box::new(MakeDir::new(Path::new(LOC_HG_UT))),
                    Box::new(MountVolumeSub::new(NAME_SV_UT, &self.partition_root, Path::new(LOC_HG_UT))),
                    Box::new(MakeDir::new(Path::new(LOC_HG_SIN))),
                    Box::new(Mount::new(&self.partition_boot, Path::new(LOC_HG_SIN))),
                ]),
                (String::from(TXT_DOWNLOAD_OS), vec![
                    Box::new(OSIndexDownload::new(URL_ARMTIX_DL, DEFAULT_OS_INIT, PathBuf::from(LOC_INSTALL), rc_install.path_os.clone()))
                ]),
                (String::from(TXT_EXTRACTING_OS), vec![
                    Box::new(TarExtractRc::new(rc_install.path_os, Path::new(LOC_HG_FOQ))),
                    Box::new(BridgeArchGap {}),
                ]),
                (String::from(TXT_USERS), vec![ 
                    Box::new(ChrootGroupmod::new(DEFAULT_USERGROUP_USER, &self.name_user)),
                    Box::new(ChrootUsermod::new(DEFAULT_USERNAME, &self.name_user, &self.name_full, 
                        &self.password_user, &self.password_root, &self.key_pub))
                ]),
                (String::from(TXT_PACKAGES_UPDATE), vec![ 
                    Box::new(PacmanUpdate {}),
                ]),
                (String::from(TXT_PACKAGES_EQSTALX_BAZ), vec![ 
                    Box::new(EqstalxPackage::deh(REGLO_PAHKEHT_BAZ)),
                ]),
                (String::from(TXT_FS_INSTALL), vec![ 
                    Box::new(Remove::new(Path::new(LOC_MKINITCPIO_STS))),
                    Box::new(EqstalxPackage::syl(REGLO_PAHKEHT_FS)),
                ]),
                (String::from(TXT_SINISJEHL_INSTALL), vec![
                    Box::new(EqstalxAqbarsjehl::nxvx()),
                    Box::new(EqstalxSinisjehl::nxvx()),
                ]),
                (String::from(TXT_FSTAB), vec![
                    Box::new(ZjenxFstab {})
                ]),
                (String::from(TXT_PACKAGES_EQSTALX_SIPLOJ), vec![ 
                    Box::new(EqstalxPackage::deh(REGLO_PAHKEHT_SXTJEQ)),
                ]),
                (String::from(TXT_PACKAGES_EQSTALX_AUR), vec![
                    Box::new(EqstalxPackage::syl(PAHKEHT_AUR_DEFO)),
                ]),
                (String::from(TXT_EDITOR), vec![ 
                    Box::new(Git::config("init.defaultBranch", "main")),
                    // Box::new(MountDevPts::new()),
                    Box::new(EqstalxEditor::new(&self.name_user, &self.keymap))
                ]),
                (String::from(TXT_OTONOMI), vec![
                    Box::new(AzjxRezosur {}),
                ]),
                (String::from(TXT_SETTINGS_SYSTEM), vec![
                    Box::new(SetSettingsSystem::new(&self.region_timezone, &self.zone_timezone, &self.locale, &self.keymap, &self.name_host))
                ]),
                (String::from(TXT_CLEANUP_INSTALL), vec![
                    // Box::new(UmountPoint::syl(Path::new(&format!("{LOC_MNT}/root/var/cache/pacman/pkg")))),
                    Box::new(CleanupInstall::new()),
                ]),
                (String::from(TXT_UMOUNT_DIRS), vec![
                    Box::new(UmountPoint::deh(&[LOC_HG_SIN, LOC_HG_UT, LOC_HG_FOQ]))
                ]),
                (String::from(TXT_PARTPROBE), vec![
                    Box::new(Partprobe::new(&self.drive))
                ])
            ],
            _ => panic!("Device type not found in list: {}", &self.device),
        }    
    }

    /*
    pub fn get_method(&self, index: u32) -> (String, Option<Expression>) {
        match self.device.as_str() {
            "rpi4" => match index {        
                0 => (String::from(TXT_UMOUNT_SD_CARD), CommandAction::umount_drive(&self.drive)),
                1 => (String::from(TXT_RM_PARTITIONS), CommandAction::remove_partitions_drive(&self.drive)),
                2 => (String::from(TXT_DD_FIRST_MBS), CommandAction::dd_first_mbs(&self.drive)),
                3 => (String::from(TXT_MKLABEL), CommandAction::make_label(&self.drive)),
                4 => (String::from(TXT_MKBOOT), CommandAction::make_boot_partition(&self.drive, TYPE_FS_FAT32)),
                5 => (String::from(TXT_MKROOT), CommandAction::make_root_partition(&self.drive, TYPE_FS_BTRFS)),
                6 => (String::from(TXT_PARTPROBE), CommandAction::partprobe(&self.drive)),
                7 => (String::from(TXT_MKVFAT), CommandAction::mkfs_vfat(&self.drive, PART_BOOT)),
                8 => (String::from(TXT_MKBTRFS), CommandAction::mkfs_btrfs(&self.drive, PART_ROOT)),
                9 => (String::from(TXT_MKLOC_MNTS), CommandAction::make_dirs(&[DIR_HG_ROOT, DIR_HG_BOOT])),
                // 10 => (String::from(TXT_MNT_MAINVOL_ROOT), CommandAction::mount_mainvol(&self.partition_root, LOC_HG_ROOT)),
                // 11 => (String::from(TXT_MKSUBVOL_ROOT), CommandAction::make_subvol(Path::new(LOC_SV_ROOT))),
                // 12 => (String::from(TXT_MKSUBVOL_HOME), CommandAction::make_subvol(Path::new(LOC_SV_HOME))),
                // 13 => (String::from(TXT_UMOUNT_ROOT), CommandAction::umount_volume(&self.partition_root)),
                // 14 => (String::from(TXT_MNT_SUBVOLS), CommandAction::mount_subvols(&self.partition_root, &SUBVOLS_PART_ROOT)),
                // 15 => (String::from(TXT_MNT_BOOT), CommandAction::mount(&self.partition_boot, LOC_HG_BOOT)),
                16 => (String::from(TXT_DOWNLOAD_OS), CommandAction::wget(LOC_MNT, format!("{URL_ARMTIX_DL}{FILE_XZ_ARMTIX}").as_str())),
                17 => (String::from(TXT_EXTRACTING_OS), CommandAction::extract_rootfs(format!("{LOC_MNT}/{FILE_XZ_ARMTIX}").as_str(), PathBuf::from(DIR_HG_ROOT))),
                18 => (String::from(TXT_BR_ARCH_GAP), CommandAction::bridge_arch_gap()),
                /*
                19 => (String::from(TXT_PACKAGES_UPDATE), CommandAction::update_packages()),
                20 => (String::from(TXT_INSTALL_BOOTLOADER_BUILDER), CommandAction::eqstalx_builder(DEFAULT_BOOTLOADER)),
                21 => (String::from(TXT_INSTALL_BOOTLOADER), CommandAction::eqstalx_bootloader()),
                22 => (String::from(TXT_FSTAB), CommandAction::zjenx_fstab()),
                23 => (String::from(TXT_MOVE_BOOT), CommandAction::move_boot()),
                24 => (String::from(TXT_PACKAGES_INSTALL), CommandAction::eqstalx_packages(DEFAULT_PACKAGES)),
                25 => (String::from(TXT_PACKAGE_FS), CommandAction::eqstalx_fs()),
                26 => (String::from(TXT_USERS), CommandAction::set_users(&self.name_user, &self.name_full, &self.password_user, &self.password_root, &self.key_pub)),
                27 => (String::from(TXT_REZOSUR), CommandAction::azjx_rezosur()),
                28 => (String::from(TXT_EDITOR), CommandAction::azjx_editor()),
                */
                19 => (String::from(TXT_SETTINGS_SYSTEM), CommandAction::set_settings_system(&self.region_timezone, &self.zone_timezone, &self.locale, &self.keymap, &self.name_host)),
                20 => (String::from(TXT_CLEAN_INSTALL), CommandAction::clean_up_install(DEFAULT_ARCH)),
                21 => (String::from(TXT_UMOUNT_DIRS), CommandAction::umount_dirs(&[LOC_HG_BOOT, DIR_HG_HOME, DIR_HG_ROOT])),
                22 => (String::from(TXT_PARTPROBE), CommandAction::partprobe(&self.drive)),
            },
            "test" => {
                match index {
                    0 => (String::from(TXT_UMOUNT_SD_CARD), CommandAction::umount_drive(&self.drive)),
                    1 => (String::from(TXT_RM_PARTITIONS), CommandAction::remove_partitions_drive(&self.drive)),
                    2 => (String::from(TXT_DD_FIRST_MBS), CommandAction::dd_first_mbs(&self.drive)),
                    3 => (String::from(TXT_MKLABEL), CommandAction::make_label(&self.drive)),
                    4 => (String::from(TXT_MKBOOT), CommandAction::make_boot_partition(&self.drive, TYPE_FS_FAT32)),
                    5 => (String::from(TXT_MKROOT), CommandAction::make_root_partition(&self.drive, TYPE_FS_BTRFS)), 
                    6 => (String::from(TXT_PARTPROBE), CommandAction::partprobe(&self.drive)),
                    7 => (String::from(TXT_MKVFAT), CommandAction::mkfs_vfat(&self.drive, PART_BOOT)),
                    8 => (String::from(TXT_MKBTRFS), CommandAction::mkfs_btrfs(&self.drive, PART_ROOT)),
                    9 => (String::from(TXT_MKLOC_MNTS), CommandAction::make_dirs(&[DIR_HG_ROOT])),
                    10 => (String::from(TXT_MNT_MAINVOL_ROOT), CommandAction::mount_drive(
                        &self.partition_root, Path::new(LOC_HG_ROOT), 
                        &[Path::new(LOC_SV_ROOT), Path::new(DIR_SV_HOME)])),
                    11 => (String::from(TXT_UMOUNT_ROOT), CommandAction::umount_volume(&self.partition_root)),
                    12 => (String::from(TXT_MNT_SUBVOLS), CommandAction::mount_drive_part_two(
                        &self.partition_root, &[
                            (Path::new(LOC_HG_ROOT), DIR_END_SV_ROOT), 
                            (Path::new(LOC_HG_HOME), DIR_END_SV_HOME)
                        ],
                        (&self.partition_boot, Path::new(LOC_HG_BOOT)))),
                    13 => (String::from(TXT_DOWNLOAD_OS), CommandAction::wget(LOC_MNT, format!("{URL_ARMTIX_DL}{FILE_XZ_ARMTIX}").as_str())),
                    14 => (String::from(TXT_EXTRACTING_OS), CommandAction::extract_rootfs(format!("{LOC_MNT}/{FILE_XZ_ARMTIX}").as_str(), PathBuf::from(DIR_HG_ROOT))),
                    15 => (String::from(TXT_BR_ARCH_GAP), CommandAction::bridge_arch_gap()),
                    16 => (String::from(TXT_PACKAGES_UPDATE), CommandAction::update_packages()),
                    17 => (String::from(TXT_USERS), CommandAction::set_users(&self.name_user, &self.name_full, &self.password_user, &self.password_root, &self.key_pub)),
                    18 => (String::from(TXT_FSTAB), CommandAction::zjenx_fstab()),
                    // 23 => (String::from(TXT_INSTALL_BOOTLOADER), CommandAction::eqstalx_bootloader(BL_UEFI, &self.drive, PART_BOOT)),
                    // 22 => (String::from(TXT_MOVE_BOOT), CommandAction::move_boot()),
                    19 => (String::from(TXT_PACKAGES_INSTALL), CommandAction::eqstalx_packages(DEFAULT_PACKAGES)),
                    20 => (String::from(TXT_PACKAGE_FS), CommandAction::eqstalx_fs()),
                    21 => (String::from(TXT_REZOSUR), CommandAction::azjx_rezosur()),
                    22 => (String::from(TXT_EDITOR), CommandAction::azjx_editor()),
                }
            }
            _ => panic!("Command function not found in list: {}, {}", &self.device, index),
        }
    */
}
