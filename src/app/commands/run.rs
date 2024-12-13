use std::path::{Path, PathBuf};

use duct::cmd;
use duct::Expression;

use reqwest;
use scraper::Html;

use crate::app::commands::read::CommandRead;
use crate::shared::constants::command::*;
use crate::shared::constants::install::*;
use crate::shared::constants::error::ErrorInstaller::FailedReadCommand; 

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
    LOC_MAHRK_IMAZJ_FINI
];

pub enum TypeCommandRun {
    Syl(Expression),
    Deh(Vec<Expression>),    
    Kuq()
}

pub trait CommandRun {
    fn prepare(&self) -> TypeCommandRun;
}

pub struct AzjxRezosur {}
impl AzjxRezosur {}

impl CommandRun for AzjxRezosur {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Kuq()
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
        TypeCommandRun::Deh(deh_cmd)
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
        match cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GETENT, ACS_GROUP, &self.group_old).stdout_capture().stderr_capture().unchecked().run() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(result_command) => {
                match result_command.status.code() {
                    Some(0) => TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GROUPMOD, ARG_N, &self.group_new, &self.group_old)),
                    _ => TypeCommandRun::Kuq(),
                }
            },
        }
    }
}

pub struct ChrootUsermod {
    pub key_pub_user: String,  
    pub name_full_user: String,  
    pub name_user_new: String, 
    pub name_user_old: String, 
    pub password_root: String, 
    pub password_user: String,  
}

impl ChrootUsermod {
    pub fn new(name_user_old: &str, name_user_new: &str, name_full_user: &str, password_user: &str, password_root: &str, key_pub_user: &str) -> ChrootUsermod {
        ChrootUsermod {
            key_pub_user: String::from(key_pub_user),
            name_full_user: String::from(name_full_user),
            name_user_new: String::from(name_user_new),
            name_user_old: String::from(name_user_old),
            password_root: String::from(password_root),
            password_user: String::from(password_user),
        }
    }
}

impl CommandRun for ChrootUsermod {
    fn prepare(&self) -> TypeCommandRun {
        let user = &self.name_user_new;
        
        match cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GETENT, ACS_PASSWD, &self.name_user_old).stdout_capture().stderr_capture().unchecked().run() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(result_command) => {
                match result_command.status.code() {
                    Some(0) => {
                        let path_auth_keys = PathBuf::from(format!("/home/{user}/.ssh/authorized_keys"));
                        let path_hg_auth_keys = PathBuf::from(format!("{DIR_HG_ROOT}/home/{user}/.ssh/authorized_keys"));
                        let path_key_pub_user = PathBuf::from(format!("/home/bas/.ssh/{}", &self.key_pub_user));
                        TypeCommandRun::Deh(vec![
                            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, USERMOD, ARG_L, user, DEFAULT_USERNAME, 
                                ARG_A, ARGS_G, DEFAULT_USERGROUPS, ARG_P, &self.password_user, ARG_S, DEFAULT_SHELL, 
                                ARG_M, ARG_D, format!("/home/{user}"), ARG_C, &self.name_full_user),
                            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, INSTALL, ARL_DIR, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD700, format!("/home/{user}/.ssh")), 
                            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD600, ACS_DEV_NULL, path_auth_keys.clone()), 
                            cmd!(ECHO, path_key_pub_user)
                                .pipe(cmd!(TEE, ARG_A, path_hg_auth_keys)), 
                            cmd!(SUDO, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD644, LOC_PROFILE, format!("{DIR_HG_ROOT}/home/{user}/.profile")), 
                            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, USERMOD, ARG_P, &self.password_root, ROOT)
                        ])
                    }
                    _ => TypeCommandRun::Kuq()
                }
            }
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
        deh_expr.push(cmd!(SUDO, RM, Path::new(&format!("{DIR_MNT}/{}-2024*", &self.init))));
        TypeCommandRun::Deh(deh_expr)
    }
}

pub struct DdFirstMbs {
    drive: PathBuf
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

pub struct EqstalxEditor {
    keymap: String,
    user: String
}

impl EqstalxEditor {
    pub fn new(user: &str, keymap: &str) -> EqstalxEditor {
        EqstalxEditor {
            keymap: keymap.into(), 
            user: user.into()
        }
    }
}

impl CommandRun for EqstalxEditor {
    fn prepare(&self) -> TypeCommandRun {
        let dir_nvim_guest = format!("{DIR_HG_ROOT}/home/{}/.config/nvim", &self.user);
        let dir_nvim_host = "home/bas/.config/nvim";
        let dir_keymaps_guest = format!("{}/keymaps", dir_nvim_guest);
        let dir_keymaps_host = format!("{}/keymaps", dir_nvim_host);
        let dir_keymap = format!("/home/bas/SE_Bastille/src/files/{}.lua", self.keymap);

        let copy_nvim = cmd!(RSYNC, ARG_A, ARL_EXCLUDE, Path::new(&dir_keymaps_guest), 
            Path::new(dir_nvim_host), Path::new(&dir_nvim_guest));

        match self.keymap.as_str() {
            "yr" =>
                TypeCommandRun::Deh(vec![
                    copy_nvim,
                    cmd!(ARTIX_CHROOT, DIR_HG_ROOT, MKDIR, Path::new(&dir_keymaps_host)),
                    cmd!(CP, Path::new(&dir_keymap), Path::new(&dir_keymaps_guest))
                ]),
            _ =>
            TypeCommandRun::Syl(copy_nvim)
        }
    }
}



pub struct EqstalxFs {}
impl EqstalxFs {}

impl CommandRun for EqstalxFs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Deh(vec![
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, RM, LOC_MKINITCPIO_STS),
            cmd!(ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_S, ARL_NOCONFIRM, DEFAULT_PACKAGE_FS)
        ])
    }
}

pub struct EqstalxPackage {
    packages: Vec<String>
}

impl EqstalxPackage {
    #[allow(dead_code)]
    pub fn syl(package: &str) -> EqstalxPackage {
        EqstalxPackage {
            packages: vec![package.into()] 
        } 
    }

    pub fn deh(packages: &[&str]) -> EqstalxPackage {
        EqstalxPackage {
            packages: packages.iter().map(|package| String::from(*package)).collect()
        }
    }
}

impl CommandRun for EqstalxPackage {
    fn prepare(&self) -> TypeCommandRun {
        let mut deh_cmd = vec![];
        if Path::new(&format!("{DIR_HG_ROOT}/{LOC_DB_LOCK_PACMAN}")).exists() {
            deh_cmd.push(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, RM, LOC_DB_LOCK_PACMAN));
        }
        self.packages.iter().for_each(|e| {
            deh_cmd.push(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, PACMAN, ARGS_S, ARL_NOCONFIRM, e));
        });
        TypeCommandRun::Deh(deh_cmd)
    }
}

#[allow(dead_code)]
pub struct EqstalxPackageAUR {
    packages: Vec<String>
}

#[allow(dead_code)]
impl EqstalxPackageAUR {
    pub fn syl(package: &str) -> EqstalxPackage {
        EqstalxPackage {
            packages: vec![package.into()] 
        } 
    }

    pub fn deh(packages: &[&str]) -> EqstalxPackage {
        EqstalxPackage {
            packages: packages.iter().map(|package| String::from(*package)).collect()
        }
    }
}

impl CommandRun for EqstalxPackageAUR {
    fn prepare(&self) -> TypeCommandRun { 
        let mut deh_cmd = vec![];
        
        self.packages.iter().for_each(|e| {
            deh_cmd.push(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, SUDO, ARG_U, TRIZEN, ARGS_S, ARL_NOCONFIRM, e));
        });
        TypeCommandRun::Deh(deh_cmd)
    }
}

pub struct Git {
    pub arg_0: String,
    pub arg_1: String,
    pub arg_2: String
}

impl Git {
    pub fn config(key: &str, value: &str) -> Git {
        Git {
            arg_0: String::from("config"),
            arg_1: key.into(),
            arg_2: value.into()
        }
    }
}

impl CommandRun for Git {
    fn prepare(&self) -> TypeCommandRun {
        let arg_0 = &self.arg_0;
        let arg_1 = &self.arg_1;
        let arg_2 = &self.arg_2;
        match arg_0.as_str() {
            "config" => TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, DIR_HG_ROOT, GIT, arg_0, ARL_GLOBAL, arg_1, arg_2)),
            _ => TypeCommandRun::Kuq(),
            
        }         
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

pub struct MountDevPts {}

impl MountDevPts {
    pub fn new() -> MountDevPts {
        MountDevPts {}
    }
}

impl CommandRun for MountDevPts {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_T, "devpts", "none", format!("{DIR_HG_ROOT}/dev/pts")))
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

    #[allow(dead_code)]
    pub fn new(path: &Path) -> Remove {
        Remove {
            path: path.into()
        }
    }

    pub fn chroot(path: &Path) -> Remove {
        Remove {
            path: Path::new(DIR_HG_ROOT).join(path)
        }
    }
}


impl CommandRun for Remove {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path;
        match (path.exists(), path.is_file(), path.is_dir()) {
            (true, true, false) => TypeCommandRun::Syl(cmd!(SUDO, RM, path)),
            (true, false, true) => TypeCommandRun::Syl(cmd!(SUDO, RM, ARG_RF, path)),
            _ => TypeCommandRun::Kuq(),
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
                .pipe(cmd!(AWK, ACS_PRINT_C1_BW_SPACE)).stderr_capture().unchecked().read() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(s) => {
                TypeCommandRun::Deh(s.lines().map(|e| {
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
        TypeCommandRun::Deh(vec![
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

    #[allow(dead_code)]
    pub fn new(path: &Path) -> Touch {
        Touch {
            path: path.into()
        }
    }

    pub fn chroot(path: &Path) -> Touch {
        Touch {
            path: Path::new(DIR_HG_ROOT).join(path)
        }
    }
}

impl CommandRun for Touch {
    fn prepare(&self) -> TypeCommandRun {
        let path = &self.path;
        match path.exists() {
            false => TypeCommandRun::Syl(cmd!(SUDO, TOUCH, path)),
            _ => TypeCommandRun::Kuq()
        }
    }
}

pub struct Umount {
    paths: Vec<PathBuf>, 
}

impl Umount {
    pub fn syl(path: &Path) -> Umount {
        Umount {
            paths: vec![path.into()]
        }
    }

    pub fn deh(paths: &[&str]) -> Umount {
        Umount {
            paths: paths.iter().map(|path| PathBuf::from(path)).collect()
        }
    }
}

impl CommandRun for Umount {
    fn prepare(&self) -> TypeCommandRun {
        let paths = &self.paths.iter().filter(|path| {
            CommandRead::is_mounted(path) 
        }).collect::<Vec<&PathBuf>>();

        match paths.len() {
            0 => TypeCommandRun::Kuq(),
            1 => TypeCommandRun::Syl(cmd!(SUDO, UMOUNT, ARG_L, paths[0])),
            _ => {
                TypeCommandRun::Deh(paths.iter().map(|e| {
                    cmd!(SUDO, UMOUNT, ARG_L, e)
                }).collect())
            }
        }
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
                    true => TypeCommandRun::Kuq(), 
                    false => panic!("{}", FailedReadCommand(format!("{e}"))),
                }
            }
            Ok(s) => {
                TypeCommandRun::Deh(s.lines().map(|e| { 
                    cmd!(SUDO, UMOUNT, e)
                }).collect())
            },
        }
    }
}

pub struct OSIndexDownload<'a> {
    os_build: String,
    path_dest: PathBuf,
    path_os: &'a mut PathBuf,
    url_index: String,
}

impl OSIndexDownload<'_> {
    pub fn new<'a>(path_os: &mut PathBuf, os_build: &'a str, url_index: &'a str, path_dest: &'a Path) -> OSIndexDownload<'a> {
        OSIndexDownload {
            path_os: &mut path_os,
            path_dest: path_dest.into(),
            os_build: os_build.into(),
            url_index: String::from(url_index),
        }
    }
}

impl CommandRun for OSIndexDownload<'_> {
    fn prepare(&self) -> TypeCommandRun {

        match reqwest::blocking::get(&self.url_index) {
            Ok(html) => {
                let doc = Html::parse_document(html);
                info!("html: {}", html);
                info!("doc: {:?}", doc);
                let path = &self.path_dest;
                *self.path_os = path;
                panic!("test");
                match path.exists() {
                    false => TypeCommandRun::Syl(cmd!(SUDO, WGET, ARG_Q, &self.url_index).dir(path)),
                    true => TypeCommandRun::Kuq(),
                }
            }
            Err(err) => panic!("Error: {}", err)
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
