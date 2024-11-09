use std::path::{Path, PathBuf};

use duct::cmd;
use duct::Expression;

use crate::app::commands::read::CommandRead;
use crate::shared::constants::command::*;
use crate::shared::constants::install::*;
use crate::shared::constants::error::ErrorInstaller::FailedReadCommand; 

// reg expressions
pub const REX_HD: &str = "'s/\\s*\\([\\+0-9a-zA-Z]*\\).*/\\1/'";

// cleanup dirs
pub const CLEANUP_CMDS: [&str; 11] = [
    "/root/var/cache/pacman/pkg",
    "/root/usr/bin/qemu-aarch64-static",
    "/root/var/cache/packman/pkg/*",
    "/root/var/log/*",
    "/root/etc/*.pacnew",
    "/root/usr/lib/systemd/system/systemd-firstboot.service",
    "/root/etc/machine-id",
    "/user",
    "/password",
    "/rootpassword",
    LOC_HG_MAHRK_IMAZJ_FINI
];

#[allow(dead_code)]
pub enum TypeCommandRun {
    Syl(Expression),
    Opt(Option<Expression>),    
    Vec(Vec<Expression>),    
}

pub trait CommandRun {
    fn prepare(&self) -> TypeCommandRun;
}

pub struct AzjxEditor {}
impl AzjxEditor {}

impl CommandRun for AzjxEditor {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, SED, ARG_E, REX_HD, FILE_LITERAL, EOF)
            .pipe(cmd!("LV_BRANCH='lelease-1.3/neovim-0.9'", CURL, ARG_S, DEFAULT_URL_EDITOR))
            .pipe(cmd!("bash", "n", "n", "y", EOF)))
    }
}

pub struct AzjxRezosur {}
impl AzjxRezosur {}

impl CommandRun for AzjxRezosur {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Opt(None)
    }
}

pub struct BridgeArchGap {}
impl BridgeArchGap {}

impl CommandRun for BridgeArchGap {
    fn prepare(&self) -> TypeCommandRun {
        let mut deh_cmd = vec![];
        if !Path::new(LOC_HG_QEMU_USER_STATIC).exists() {
            deh_cmd.push(cmd!(SUDO, INSTALL, ARG_MOD755, ARGS_C, LOC_QEMU_USER_STATIC, LOC_HG_QEMU_USER_STATIC));
            if !Path::new(LOC_DEFAULT_BINFMT_ARCH).exists() {
                deh_cmd.push(cmd!(CAT, LOC_BINFMT_AARCH64).pipe(cmd!(TEE, LOC_BINFMT_REGISTER)));
            }
        }
        TypeCommandRun::Vec(deh_cmd)
    }
}

pub struct ChrootGroupmod {
    pub group_new: String, 
    pub group_old: String
}

impl ChrootGroupmod {
    pub fn new(group_old: &str, group_new: &str) -> ChrootGroupmod {
        ChrootGroupmod {
            group_new: String::from(group_new),
            group_old: String::from(group_old)
        }
    }
}

impl CommandRun for ChrootGroupmod {
    fn prepare(&self) -> TypeCommandRun {
        match cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GETENT, &self.group_old).read() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(s) => {
                match s.is_empty() {
                    false => TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GROUPMOD, ARG_N, &self.group_new, &self.group_old)),
                    true => TypeCommandRun::Opt(None),
                }
            },
        }
    }
}

pub struct CleanupInstall {
    init: String
}

impl CleanupInstall {
    pub fn new(init: &str) -> CleanupInstall {
        CleanupInstall {
            init: init.into()
        }
    }
}

impl CommandRun for CleanupInstall {
    fn prepare(&self) -> TypeCommandRun {
        let mut deh_expr = CLEANUP_CMDS.iter().map(|e| 
            cmd!(SUDO, RM, Path::new(&format!("{DIR_MNT}/{}", e)))).collect::<Vec<Expression>>();
        deh_expr.push(cmd!(SUDO, RM, Path::new(&format!("{DIR_MNT}/armtix-{}-2024*", &self.init))));
        TypeCommandRun::Vec(deh_expr)
    }
}

pub struct DdFirstMbs {
    drive: PathBuf,
}

impl DdFirstMbs {
    pub fn new(drive: &Path) -> DdFirstMbs {
        DdFirstMbs {
            drive: drive.into()
        }
    }
}

impl CommandRun for DdFirstMbs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, DD, IF_DEV_ZERO, format!("of={}", self.drive.display()), BS_1M, COUNT_32, STATUS_NONE))
    }
}

pub struct EqstalxFs {}
impl EqstalxFs {}

impl CommandRun for EqstalxFs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Vec(vec![
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, RM, LOC_MKINITCPIO_STS),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_S, ARL_NOCONFIRM, DEFAULT_PACKAGE_FS)
        ])
    }
}

pub struct EqstalxPackages {
    pub packages: String
}

impl EqstalxPackages {
    pub fn new(packages: &str) -> EqstalxPackages {
        EqstalxPackages {
            packages: String::from(packages),
        }
    }
}

impl CommandRun for EqstalxPackages {
    fn prepare(&self) -> TypeCommandRun {
        let mut deh_cmd = vec![];
        if Path::new(&format!("{DIR_HG_ROOT}/{LOC_DB_LOCK_PACMAN}")).exists() {
            deh_cmd.push(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, RM, LOC_DB_LOCK_PACMAN));
        }
        deh_cmd.push(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_S, ARL_NOCONFIRM, &self.packages));
        TypeCommandRun::Vec(deh_cmd)
    }
}

pub struct MakeDir {
    pub path: PathBuf
}

impl MakeDir {
    pub fn new(path: &Path) -> MakeDir {
        MakeDir {
            path: path.into()
        }
    }
}

impl CommandRun for MakeDir {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MKDIR, ARG_P, &self.path))
    }
}

pub struct MakeLabel {
    pub filesystem: PathBuf  
}

impl MakeLabel {
    pub fn new(filesystem: &Path) -> MakeLabel {
        MakeLabel {
            filesystem: filesystem.into()
        }
    }
}


impl CommandRun for MakeLabel {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, ARG_S, &self.filesystem, MKLABEL, GPT))
    }
}

pub struct MakePartitionBoot {
    pub filesystem: PathBuf,
    pub format: String
}

impl MakePartitionBoot {
    pub fn new(filesystem: &Path, format: &str) -> MakePartitionBoot {
        MakePartitionBoot {
            filesystem: filesystem.into(),
            format: String::from(format)
        }
    }
}

impl CommandRun for MakePartitionBoot {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, ARG_A, OPTIMAL, ARG_S, &self.filesystem, MKPART, PRIMARY, &self.format, ACS_1MIB, ACS_BOOT_SPACE))
    }
}

pub struct MakePartitionRoot {
    pub filesystem: PathBuf,
    pub format: String
}

impl MakePartitionRoot {
    pub fn new(filesystem: &Path, format: &str) -> MakePartitionRoot {
        MakePartitionRoot {
            filesystem: filesystem.into(),
            format: String::from(format)
        }
    }
}

impl CommandRun for MakePartitionRoot {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, ARG_A, OPTIMAL, ARG_S, &self.filesystem, MKPART, PRIMARY, &self.format, ACS_BOOT_SPACE, C_PERCENT))
    }
}

pub struct MkfsBtrfs {
    pub filesystem: PathBuf,
    pub partition: u32
}

impl MkfsBtrfs {
    pub fn new(filesystem: &Path, partition: u32) -> MkfsBtrfs {
        MkfsBtrfs {
            filesystem: filesystem.into(),
            partition
        }
    }
}

impl CommandRun for MkfsBtrfs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MKFS_BTRFS, ARG_M, SINGLE, ARGS_L, LABEL_ROOT_AND_HOME, ARG_F, format!("{}{}", 
            &self.filesystem.display(), &self.partition)))
    }
}

pub struct MkfsBtrfsSub {
    pub mountpoint: PathBuf,
}

impl MkfsBtrfsSub {
    pub fn new(mountpoint: &Path) -> MkfsBtrfsSub {
        MkfsBtrfsSub {
            mountpoint: mountpoint.into(),
        }
    }
}

impl CommandRun for MkfsBtrfsSub {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, BTRFS, ACS_SU, ACS_CR, &self.mountpoint))
    }
}

pub struct MkfsVfat {
    pub filesystem: PathBuf, 
    pub partition: u32  
}

impl MkfsVfat {
    pub fn new(filesystem: &Path, partition: u32) -> MkfsVfat {
        MkfsVfat {
            filesystem: filesystem.into(),
            partition
        }
    }
}

impl CommandRun for MkfsVfat {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MKFS_VFAT, ARG_N, LABEL_BOOT, format!("{}{}", 
            &self.filesystem.display(), &self.partition)))
    }
}

pub struct Mount {
    pub filesystem: PathBuf, 
    pub mountpoint: PathBuf
}

impl Mount {
    pub fn new(filesystem: &Path, mountpoint: &Path) -> Mount {
        Mount {
            filesystem: filesystem.into(),
            mountpoint: mountpoint.into()
        }
    }
}

impl CommandRun for Mount {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, &self.filesystem, &self.mountpoint))
    }
}

pub struct MountVolumeMain {
    pub filesystem: PathBuf,
    pub mountpoint: PathBuf,
}

impl MountVolumeMain {
    pub fn new(filesystem: &Path, mountpoint: &Path) -> Mount {
        Mount {
            filesystem: filesystem.into(),
            mountpoint: mountpoint.into()
        }
    }
}

impl CommandRun for MountVolumeMain {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_O, MAIN_VOL_COMPRESS, &self.filesystem, &self.mountpoint))
    }
}

pub struct MountVolumeSub {
    pub name: String,
    pub filesystem: PathBuf,
    pub mountpoint: PathBuf,
}

impl MountVolumeSub {
    pub fn new(name: &str, filesystem: &Path, mountpoint: &Path) -> MountVolumeSub {
        MountVolumeSub {
            name: String::from(name), 
            filesystem: filesystem.into(),
            mountpoint: mountpoint.into()
        }
    }
}

impl CommandRun for MountVolumeSub {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_O, format!("{SUB_VOL_COMPRESS}{}", &self.name), &self.filesystem, &self.mountpoint))
    }
}

pub struct PacmanUpdate {}
impl PacmanUpdate {}

impl CommandRun for PacmanUpdate {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_SYYU, ARL_NOCONFIRM))
    }
}

pub struct Partprobe {
    pub filesystem: PathBuf  
}

impl Partprobe {
    pub fn new(filesystem: &Path) -> Partprobe {
        Partprobe {
            filesystem: filesystem.into()
        }
    }
}

impl CommandRun for Partprobe {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, PARTPROBE, &self.filesystem))
    }
}


pub struct Remove {
    pub path: PathBuf  
}

impl Remove {
    pub fn new(path: &Path) -> Remove {
        Remove {
            path: path.into()
        }
    }
}


impl CommandRun for Remove {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path;
        match (path.exists(), path.is_file(), path.is_dir()) {
            (true, true, false) => TypeCommandRun::Syl(cmd!(SUDO, RM, path)),
            (true, false, true) => TypeCommandRun::Syl(cmd!(SUDO, RM, ARG_RF, path)),
            _ => TypeCommandRun::Opt(None),
        } 
    }
}


pub struct RemovePartitionsDrive {
    pub filesystem: PathBuf,
}

impl RemovePartitionsDrive {
    pub fn new(filesystem: &Path) -> RemovePartitionsDrive {
        RemovePartitionsDrive {
            filesystem: filesystem.into()
        }
    }
}

impl CommandRun for RemovePartitionsDrive { 
    fn prepare(&self) -> TypeCommandRun {
        match cmd!(SUDO, PARTED, &self.filesystem, ACS_PRINT)
                .pipe(cmd!(AWK, ACS_PRINT_C1_BW_SPACE)).read() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(s) => {
                TypeCommandRun::Vec(s.lines().map(|e| {
                    cmd!(SUDO, PARTED, ARG_S, &self.filesystem, RM, e)
                }).collect())
            },
        }
    }
}

pub struct SetSettingsSystem {
    pub keymap: String,
    pub locale: String,
    pub name_host: String,
    pub timezone_uqkeh: String,
    pub timezone_dykeh: String,
}

impl SetSettingsSystem {
    pub fn new(timezone_region: &str, timezone_zone: &str, locale: &str, keymap: &str, name_host: &str) -> SetSettingsSystem {
        SetSettingsSystem {
            keymap: String::from(keymap),
            locale: String::from(locale),
            name_host: String::from(name_host),
            timezone_uqkeh: String::from(timezone_region),
            timezone_dykeh: String::from(timezone_zone),
        }
    }
}

impl CommandRun for SetSettingsSystem {
    fn prepare(&self) -> TypeCommandRun {
        let locale = &self.locale;
        TypeCommandRun::Vec(vec![
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, LN, ARG_S, ARG_F, format!("/usr/share/zoneinfo/timezone/{}/{}",
                &self.timezone_uqkeh, &self.timezone_dykeh), "/etc/localtime"),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, SED, ARG_I, format!("s/\"#{locale}\"/\"{locale}\"/g"), LOC_LOCALE_GEN),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, ECHO, format!("\"#LOCALE={locale}\"")).pipe(cmd!(TEE, ARG_A, LOC_LOCALE_CONF)),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, LOCALE_GEN),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, ECHO, format!("\"KEYMAP={}\nFONT={DEFAULT_CONSOLEFONT}\"", 
                &self.keymap)).pipe(cmd!(TEE, ARG_A, LOC_VCONSOLE_CONF)),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, LOCALE_GEN),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, &self.name_host).pipe(cmd!(TEE, ARG_A, LOC_HOSTNAME))
        ])
    }
}


pub struct SetUsers {
    pub key_pub_user: String,  
    pub name_full_user: String,  
    pub name_user: String, 
    pub password_root: String, 
    pub password_user: String,  
}

impl SetUsers {
    pub fn new(key_pub_user: &str, name_full_user: &str, name_user: &str, password_root: &str, password_user: &str) -> SetUsers {
        SetUsers {
            key_pub_user: String::from(key_pub_user),
            name_full_user: String::from(name_full_user),
            name_user: String::from(name_user),
            password_root: String::from(password_root),
            password_user: String::from(password_user),
        }
    }
}

impl CommandRun for SetUsers {
    fn prepare(&self) -> TypeCommandRun {
        let user = &self.name_user;
        TypeCommandRun::Vec(vec![
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, USERMOD, ARG_L, user, DEFAULT_USERNAME, 
                ARG_A, ARGS_G, DEFAULT_USERGROUPS, ARG_P, &self.password_user, ARG_S, DEFAULT_SHELL, 
                ARG_M, ARG_D, format!("/home/{user}"), ARG_C, &self.name_full_user),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, INSTALL, ARL_DIR, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), ARG_MOD700, format!("/home/{user}/.ssh")), 
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), ARG_MOD600, 
                format!("<({ECHO} {})", &self.key_pub_user), format!("/home/{user}/.ssh/authorized_keys")), 
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), ARG_MOD644, LOC_PROFILE, format!("/home/{user}/.profile")), 
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, USERMOD, ARG_P, &self.password_root, ROOT)
        ])
    }
}


pub struct TarExtract {
    pub path_from: PathBuf,  
    pub path_to: PathBuf  
}

impl TarExtract {
    pub fn new(path_from: &Path, path_to: &Path) -> TarExtract {
        TarExtract {
            path_from: path_from.into(),
            path_to: path_to.into()
        }
    }
}

impl CommandRun for TarExtract {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, TAR, ARG_XF, &self.path_from, ARGS_C, &self.path_to))
    }
}

pub struct Touch {
    pub path: PathBuf  
}

impl Touch {
    pub fn new(path: &Path) -> Remove {
        Remove {
            path: path.into()
        }
    }
}

impl CommandRun for Touch {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path;
        match path.exists() {
            false => TypeCommandRun::Syl(cmd!(SUDO, TOUCH, path)),
            _ => TypeCommandRun::Opt(None)
        }
    }
}

pub struct Umount {
    pub path: PathBuf, 
}

impl Umount {
    pub fn new(path: &Path) -> Umount {
        Umount {
            path: path.into()
        }
    }
}

impl CommandRun for Umount {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, UMOUNT, &self.path))
    }
}

pub struct UmountDirs {
    pub paths: Vec<PathBuf>, 
}

impl UmountDirs {
    pub fn new(dirs: &[&str]) -> UmountDirs {
        UmountDirs {
            paths: dirs.iter().map(|dir| PathBuf::from(dir)).collect(),
        }
    }
}

impl CommandRun for UmountDirs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Vec(self.paths.iter().map(|path| { 
            cmd!(SUDO, UMOUNT, path)
        }).collect())
    }
}


pub struct UmountDrive {
    pub filesystem: PathBuf,
}

impl UmountDrive {
    pub fn new(filesystem: &Path) -> UmountDrive {
        UmountDrive {
            filesystem: filesystem.into()
        }
    }
}

impl CommandRun for UmountDrive {
    fn prepare(&self) -> TypeCommandRun {
        match cmd!(SH, ARG_C, format!("{LSBLK} {ARG_NO} {ACS_MOUNTPOINTS} {ARG_LP} {ARL_FILTER} \
            'NAME =~ \"{}[0-9]*\" && MOUNTPOINTS =~ \"..*\"'", self.filesystem.display()))
            .read() {
            Err(e) => {
                match e.to_string().ends_with("exited with code 32") {
                    true => TypeCommandRun::Opt(None), 
                    false => panic!("{}", FailedReadCommand(format!("{e}"))),
                }
            }
            Ok(s) => {
                TypeCommandRun::Vec(s.lines().map(|e| { 
                    cmd!(SUDO, UMOUNT, e)
                }).collect())
            },
        }
    }
}

pub struct UmountVolume {
    path: PathBuf
}

impl UmountVolume {
    pub fn new(path: &Path) -> UmountVolume {
        UmountVolume {
            path: path.into()
        }
    }
}

impl CommandRun for UmountVolume {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path; 
        match CommandRead::is_mounted(path) {
            true => TypeCommandRun::Syl(cmd!(SUDO, UMOUNT, path)),
            false => TypeCommandRun::Opt(None),
        }
    }
}

pub struct Wget {
    path: PathBuf,
    url_download: String
}

impl Wget {
    pub fn new(url_download: &str, path: &Path) -> Wget {
        Wget {
            path: path.into(),
            url_download: String::from(url_download)
        }
    }
}

impl CommandRun for Wget {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path; 
        match path.exists() {
            false => TypeCommandRun::Syl(cmd!(SUDO, WGET, ARG_Q, &self.url_download).dir(path)),
            true => TypeCommandRun::Opt(None),
        }
    }
}


pub struct ZjenxFstab {}
impl ZjenxFstab {}

impl CommandRun for ZjenxFstab {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, FSTABGEN, ARGS_U, DIR_HG_ROOT).pipe(cmd!(SUDO, TEE, LOC_HG_FSTAB)))
    }
}





    /*
#[derive(Clone, Copy)]
pub struct CommandAction {}
    
impl CommandAction {


    pub fn pacman_update() -> Vec<Expression> {
        vec![cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_SYYU, ARL_NOCONFIRM)]
    }
}
*/
