use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use const_format::concatcp;

use duct::cmd;
use duct::Expression;

// use glob::glob;

use reqwest;
use scraper::{Html, selector::Selector};

use crate::app::commands::read::CommandRead;
use crate::app::dbox::r#type::Page;
use crate::shared::constants::command::*;
use crate::shared::constants::error::ErrorInstaller::FailedReadCommand; 
use crate::shared::constants::install::*;
use crate::shared::constants::scraper::*;

// cleanup dirs
pub const CLEANUP_CMDS: [&str; 7] = [
    concatcp!(LOC_HG_FOQ, "/var/cache/pacman/pkg"),
    concatcp!(LOC_HG_FOQ, "/usr/bin/qemu-aarch64-static"),
    concatcp!(LOC_HG_FOQ, "/var/cache/packman/pkg/*"),
    concatcp!(LOC_HG_FOQ, "/var/log/*"),
    concatcp!(LOC_HG_FOQ, "/etc/*.pacnew"),
    concatcp!(LOC_HG_FOQ, "/usr/lib/systemd/system/systemd-firstboot.service"),
    concatcp!(LOC_HG_FOQ, "/etc/machine-id"),
];

pub enum TypeCommandRun {
    Syl(Expression),
    Deh(Vec<Expression>),    
    Kuq(),
    Ehryr(Page)
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
        match cmd!(ARTIX_CHROOT, LOC_HG_FOQ, GETENT, ACS_GROUP, &self.group_old).stdout_capture().stderr_capture().unchecked().run() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(result_command) => {
                match result_command.status.code() {
                    Some(0) => TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, GROUPMOD, ARG_N, &self.group_new, &self.group_old)),
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
        
        match cmd!(ARTIX_CHROOT, LOC_HG_FOQ, GETENT, ACS_PASSWD, &self.name_user_old).stdout_capture().stderr_capture().unchecked().run() {
            Err(e) => panic!("{}", FailedReadCommand(format!("{e}"))),
            Ok(result_command) => {
                match result_command.status.code() {
                    Some(0) => {
                        let path_auth_keys = PathBuf::from(format!("/home/{user}/.ssh/authorized_keys"));
                        let path_hg_auth_keys = PathBuf::from(format!("{LOC_HG_FOQ}/home/{user}/.ssh/authorized_keys"));
                        let path_key_pub_user = PathBuf::from(format!("/home/bas/.ssh/{}", &self.key_pub_user));
                        TypeCommandRun::Deh(vec![
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, USERMOD, ARG_L, user, DEFAULT_USERNAME, 
                                ARG_A, ARGS_G, DEFAULT_USERGROUPS, ARG_P, &self.password_user, ARG_S, DEFAULT_SHELL, 
                                ARG_M, ARG_D, format!("/home/{user}"), ARG_C, &self.name_full_user),
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, INSTALL, ARL_DIR, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD700, format!("/home/{user}/.ssh")), 
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD600, ACS_DEV_NULL, path_auth_keys.clone()), 
                            cmd!(ECHO, path_key_pub_user)
                                .pipe(cmd!(TEE, ARG_A, path_hg_auth_keys)),
                            cmd!(RSYNC, LOC_BASHRC_USER, LOC_HG_VAR_TMP), 
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, INSTALL, format!("{ARL_OWNER_IS}{RAS}"), format!("{ARL_GROUP_IS}{RAS}"), 
                                ARG_MOD644, LOC_TMP_BASHRC, LOC_BASHRC_ROOT), 
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, INSTALL, format!("{ARL_OWNER_IS}{user}"), format!("{ARL_GROUP_IS}{user}"), 
                                ARG_MOD644, LOC_TMP_BASHRC, format!("/home/{user}/.bashrc")), 
                            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, USERMOD, ARG_P, &self.password_root, RAS)
                        ])
                    }
                    _ => TypeCommandRun::Kuq()
                }
            }
        }
    }
}


pub struct CleanupInstall {}

impl CleanupInstall {
    pub fn new() -> CleanupInstall {
        CleanupInstall {}
    }
}

impl CommandRun for CleanupInstall {
    fn prepare(&self) -> TypeCommandRun {
        let mut deh_expr = CLEANUP_CMDS.iter().map(|e| 
            cmd!(SUDO, RM, ARG_RF, Path::new(e))).collect::<Vec<Expression>>();

        // remove empty var tmp dirs
        for dir in VAR_TMP_DIRS {
            deh_expr.push(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, FIND, dir, ARG_TYPE, ACS_D, ARG_EMPTY, ARG_DELETE));
        }

        // remove var tmp files
        for file in VAR_TMP_FILES {
            deh_expr.push(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, FIND, LOC_VAR_TMP, ARG_TYPE, ACS_F, ARG_NAME, file, ARG_DELETE));
        }
     
        // for entry in glob(format!("{LOC_MNT}/*.tar.xz").as_str()).expect("Failed to read glob pattern") {
        //    match entry {
        //        Ok(path) => deh_expr.push(cmd!(SUDO, RM, path)),
        //        Err(_) => {},
        //    } 
        // }
        
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
        TypeCommandRun::Syl(cmd!(SUDO, DD, ACS_IF_DEV_ZERO, format!("of={}", self.drive.display()), ACS_BS_1M, ACS_COUNT_32, ACS_STATUS_NONE))
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
        let loc_config = format!("/home/{}/.config", &self.user);
        let loc_nvim_guest = format!("{}/nvim", loc_config);
        
        let loc_keymap_guest = format!("{}/{}.lua", LOC_VAR_TMP, self.keymap);
        let loc_keymap_host = format!("/home/bas/SE_Bastille/src/files/{}.lua", self.keymap);
        let loc_keymaps_host = format!("{}/plugins/keymaps", LOC_NVIM_HOST);
        let loc_keymaps_guest = format!("{}/plugins/keymaps", loc_nvim_guest);

        let mkdir_config = cmd!(ARTIX_CHROOT, LOC_HG_FOQ, INSTALL, ARL_DIR, 
                    format!("{ARL_OWNER_IS}{}", &self.user), 
                   format!("{ARL_GROUP_IS}{}", &self.user), ARG_MOD755, &loc_config);
        let copy_host_nvim_tmp = cmd!(RSYNC, ARG_A, ARL_EXCLUDE, Path::new(&loc_keymaps_host),
            Path::new(LOC_NVIM_HOST), Path::new(LOC_HG_VAR_TMP));
        let copy_guest_tmp_nvim = cmd!(ARTIX_CHROOT, LOC_HG_FOQ, RSYNC, ARG_A, ARL_RSF,
            format!("--chown={user}:{user}", user = &self.user), 
            Path::new(LOC_TMP_NVIM_C), Path::new(&loc_nvim_guest));
        let copy_host_keymap_tmp = cmd!(RSYNC, ARG_A, Path::new(&loc_keymap_host), Path::new(&LOC_HG_VAR_TMP));
        let copy_guest_tmp_keymap = cmd!(ARTIX_CHROOT, LOC_HG_FOQ, RSYNC, ARG_A, ARL_RSF,
            format!("--chown={user}:{user}", user = &self.user), 
            Path::new(&loc_keymap_guest), Path::new(&loc_keymaps_guest));
 
        match self.keymap.as_str() {
            "yr" =>
            TypeCommandRun::Deh(vec![
                mkdir_config,
                copy_host_nvim_tmp, 
                copy_guest_tmp_nvim,
                copy_host_keymap_tmp,
                copy_guest_tmp_keymap
            ]),
            _ =>
            TypeCommandRun::Deh(vec![
                mkdir_config,
                copy_host_nvim_tmp,
                copy_guest_tmp_nvim
            ]),
        }
   }
}

pub struct EqstalxSinisjehl {}
impl EqstalxSinisjehl {
    pub fn nxvx() -> EqstalxSinisjehl {
        EqstalxSinisjehl {}
    }
}

impl CommandRun for EqstalxSinisjehl {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Deh(vec![
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, PACMAN, ARGS_S, ARL_NOCONFIRM, REGLO_PAHKEHT_SINISJEHL),
            cmd!(RSYNC, ARG_A, LOC_FILES_BOOT, LOC_HG_VAR_TMP),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, RSYNC, ARG_A, ARL_RSF, ARL_CHOWN_ROOT, LOC_TMP_SIN_C, LOC_SIN),
        ])
    }
}

pub struct EqstalxAqbarsjehl {}
impl EqstalxAqbarsjehl {
    pub fn nxvx() -> EqstalxAqbarsjehl {
        EqstalxAqbarsjehl {}
    }
}

impl CommandRun for EqstalxAqbarsjehl {
    fn prepare(&self) -> TypeCommandRun {
        info!("pacman conf exists: {}", Path::new(&format!("{LOC_PACMAN_CONF}")).exists());
        info!("mirrorlist exists: {}", Path::new(&format!("{LOC_MIRRORLIST}")).exists());
        TypeCommandRun::Kuq()
    }
}



pub struct EqstalxFs {}
impl EqstalxFs {}

impl CommandRun for EqstalxFs {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Deh(vec![
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, RM, LOC_MKINITCPIO_STS),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, PACMAN, ARGS_S, ARL_NOCONFIRM, REGLO_PAHKEHT_FS)
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
        if Path::new(&format!("{LOC_HG_FOQ}/{LOC_DB_LOCK_PACMAN}")).exists() {
            deh_cmd.push(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, RM, LOC_DB_LOCK_PACMAN));
        }
        self.packages.iter().for_each(|e| {
            deh_cmd.push(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, PACMAN, ARGS_S, ARL_NOCONFIRM, e));
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
            deh_cmd.push(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, SUDO, ARG_U, TRIZEN, ARGS_S, ARL_NOCONFIRM, e));
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
            "config" => TypeCommandRun::Syl(cmd!(ARTIX_CHROOT, LOC_HG_FOQ, GIT, arg_0, ARL_GLOBAL, arg_1, arg_2)),
            _ => TypeCommandRun::Kuq(),
            
        }         
    }
}

/*
pub struct Mahrk {
    pub index: usize
}

impl Mahrk {
    pub fn new(index: usize) -> Mahrk {
        Mahrk {
            index: index.into(),
        }
    }
}

impl CommandRun for Mahrk {
    fn prepare(&self) -> TypeCommandRun {
        let index = self.index;
        match index {
            0 => TypeCommandRun::Syl(cmd!(SUDO, TOUCH, MAHRK_PROGREHSJOQ[0].0)),
            n if index < MAHRK_PROGREHSJOQ.len() => TypeCommandRun::Syl(cmd!(SUDO, MOVE, MAHRK_PROGREHSJOQ[n-1].0, MAHRK_PROGREHSJOQ[n].0)),
            n => TypeCommandRun::Syl(cmd!(SUDO, REMOVE, MAHRK_PROGREHSJOQ[n].0))
        }
    }
}
*/

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
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, ARG_S, &self.filesystem, MKLABEL, ACS_GPT))
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
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, &self.filesystem, 
                ARG_A, ACS_OPTIMAL, ACS_UNIT, ACS_MIB, MKPART, &self.format, 
                N_BOOT_START.to_string(), N_BOOT_SPACE.to_string()))
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
        TypeCommandRun::Syl(cmd!(SUDO, PARTED, &self.filesystem, ARG_A, ACS_OPTIMAL, ACS_UNIT, ACS_MIB, MKPART, &self.format, 
                N_BOOT_SPACE.to_string(), ACS_C_PERCENT))
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
        TypeCommandRun::Syl(cmd!(SUDO, MKFS_BTRFS, ARG_M, ACS_SINGLE, ARGS_L, LABEL_FUT, ARG_F, format!("{}{}", 
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
        TypeCommandRun::Syl(cmd!(SUDO, MKFS_VFAT, ARG_N, LABEL_SIN, format!("{}{}", 
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

/*
pub struct MountDevPts {}


impl MountDevPts {
    pub fn new() -> MountDevPts {
        MountDevPts {}
    }
}

impl CommandRun for MountDevPts {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_T, "devpts", "none", format!("{LOC_HG_FOQ}/dev/pts")))
    }
}
*/

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
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_O, ACS_MAIN_VOL_COMPRESS, &self.filesystem, &self.mountpoint))
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
        TypeCommandRun::Syl(cmd!(SUDO, MOUNT, ARG_O, format!("{ACS_SUB_VOL_COMPRESS}{}", &self.name), &self.filesystem, &self.mountpoint))
    }
}

pub struct PacmanUpdate {}
impl PacmanUpdate {}

impl CommandRun for PacmanUpdate {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, ARTIX_CHROOT, LOC_HG_FOQ, PACMAN, ARGS_SYYU, ARL_NOCONFIRM))
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

    #[allow(dead_code)]
    pub fn chroot(path: &Path) -> Remove {
        Remove {
            path: Path::new(LOC_HG_FOQ).join(path)
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
    pub drive: PathBuf,
}

impl RemovePartitionsDrive {
    pub fn new(drive: &Path) -> RemovePartitionsDrive {
        RemovePartitionsDrive {
            drive: drive.into()
        }
    }
}

impl CommandRun for RemovePartitionsDrive { 
    fn prepare(&self) -> TypeCommandRun {
        let drive = &self.drive;

        let partitions = CommandRead::partitions_drive(drive.file_name()
            .unwrap_or_else(|| panic!("Cannot read filename {}", drive.display()))
                .to_str().unwrap_or_else(|| panic!("Cannot parse filename: {}", drive.display())));

        match partitions.len() {
            0 => TypeCommandRun::Kuq(),
            1 => TypeCommandRun::Syl(cmd!(SUDO, PARTED, ARG_S, drive, RM, &partitions[0][1])),
            _ => {
                TypeCommandRun::Deh(partitions.iter().map(|e| {
                    cmd!(SUDO, PARTED, ARG_S, drive, RM, &e[1])
                }).collect())
            }
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
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, LN, ARG_S, ARG_F, format!("/usr/share/zoneinfo/timezone/{}/{}",
                &self.timezone_uqkeh, &self.timezone_dykeh), "/etc/localtime"),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, SED, ARG_I, format!("s/\"#{locale}\"/\"{locale}\"/g"), LOC_LOCALE_GEN),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, ECHO, format!("\"#LOCALE={locale}\"")).pipe(cmd!(SUDO, TEE, ARG_A, LOC_LOCALE_CONF)),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, LOCALE_GEN),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, ECHO, format!("\"KEYMAP={}\nFONT={REGLO_CONSOLEFONT}\"", 
                &self.keymap)).pipe(cmd!(SUDO, TEE, ARG_A, LOC_VCONSOLE_CONF)),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, LOCALE_GEN),
            cmd!(ARTIX_CHROOT, LOC_HG_FOQ, ECHO, &self.name_host).pipe(cmd!(SUDO, TEE, ARG_A, LOC_HOSTNAME))
        ])
    }
}

pub struct TarExtractRc {
    pub path_from: Rc<RefCell<PathBuf>>,  
    pub path_to: PathBuf  
}

impl TarExtractRc {
    pub fn new(path_from: Rc<RefCell<PathBuf>>, path_to: &Path) -> TarExtractRc {
        TarExtractRc {
            path_from,
            path_to: path_to.into()
        }
    }
}

impl CommandRun for TarExtractRc {
    fn prepare(&self) -> TypeCommandRun {
        TypeCommandRun::Syl(cmd!(SUDO, TAR, ARG_XF, 
                self.path_from.borrow().clone(), ARGS_C, &self.path_to))
    }
}

pub struct UmountPartition {
    paths: Vec<PathBuf>, 
}

impl UmountPartition {
    pub fn syl(path: &Path) -> UmountPartition {
        UmountPartition {
            paths: vec![path.into()]
        }
    }

    #[allow(dead_code)]
    pub fn deh(paths: &[&str]) -> UmountPartition {
        UmountPartition {
            paths: paths.iter().map(|path| PathBuf::from(path)).collect()
        }
    }
}

impl CommandRun for UmountPartition {
    fn prepare(&self) -> TypeCommandRun {
        let paths = &self.paths.iter().filter(|path| {
            CommandRead::is_mounted_partition(path.file_name().unwrap_or_else(|| panic!("Cannot read filename {}", path.display()))
                .to_str().unwrap_or_else(|| panic!("Cannot parse filename: {}", path.display()))) 
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

pub struct UmountPoint {
    paths: Vec<PathBuf>, 
}

impl UmountPoint {
    
    #[allow(dead_code)]
    pub fn syl(path: &Path) -> UmountPoint {
        UmountPoint {
            paths: vec![path.into()]
        }
    }

    pub fn deh(paths: &[&str]) -> UmountPoint {
        UmountPoint {
            paths: paths.iter().map(|path| PathBuf::from(path)).collect()
        }
    }
}

impl CommandRun for UmountPoint {
    fn prepare(&self) -> TypeCommandRun {
        let paths = CommandRead::points_mounted(&self.paths); 

        match paths.len() {
            0 => TypeCommandRun::Kuq(),
            1 => TypeCommandRun::Syl(cmd!(SUDO, UMOUNT, ARG_L, paths[0].clone())),
            _ => {
                TypeCommandRun::Deh(paths.iter().map(|e| {
                    cmd!(SUDO, UMOUNT, ARG_L, e)
                }).collect())
            }
        }
    }
}

pub struct UmountDrive {
    pub drive: PathBuf,
}

impl UmountDrive {
    pub fn new(drive: &Path) -> UmountDrive {
        UmountDrive {
            drive: drive.into()
        }
    }
}

impl CommandRun for UmountDrive {
    fn prepare(&self) -> TypeCommandRun {
        match CommandRead::drive_exists(&self.drive) { 
           true => { 
                match CommandRead::mountpoints_drive(&self.drive) {
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
           false => TypeCommandRun::Ehryr(Page::NotFoundDevice)
        }
    }
}

pub struct OSIndexDownload {
    a_href: Selector,
    os_build: String,
    path_dest: PathBuf,
    path_os: Rc<RefCell<PathBuf>>,
    url_index: String,
}

impl OSIndexDownload {
    pub fn new(url_index: &str, os_build: &str, path_dest: PathBuf, path_os: Rc<RefCell<PathBuf>>) -> OSIndexDownload {
        OSIndexDownload {
            a_href: Selector::parse("a").unwrap(),
            os_build: os_build.into(),
            path_dest: path_dest.into(),
            path_os,
            url_index: String::from(url_index),
        }
    }
}

impl CommandRun for OSIndexDownload {
    fn prepare(&self) -> TypeCommandRun {
    let mut path_os: PathBuf;

        match reqwest::blocking::get(&self.url_index) {
            Ok(resp) => {
                match resp.status().is_success() {
                    true => {
                        path_os = self.path_dest.clone();
                        let doc = Html::parse_document(resp.text().unwrap().as_str());
                        let opt_os_link = doc.select(&self.a_href).into_iter().find(|s| {
                            match s.value().attr(HREF) {
                                Some(attr) => {
                                    attr.starts_with(&self.os_build)  
                                },
                                #[allow(non_snake_case)]
                                None => false,
                            }
                        });

                        #[allow(non_snake_case)]
                        match opt_os_link {
                            Some(os_link) => {
                                let opt_os_filename = os_link.value().attr(HREF);
                                match opt_os_filename {
                                    Some(os_filename) => {
                                        path_os.push(os_filename);
                                        self.path_os.borrow_mut().push(path_os.clone());
                                        match path_os.exists() {
                                            false => return TypeCommandRun::Syl(cmd!(SUDO, WGET, ARG_Q, 
                                                    self.url_index.clone() + os_filename).dir(&self.path_dest)),
                                            true => return TypeCommandRun::Kuq(),
                                        }
                                    },
                                    None => {
                                        panic!("No filename found");
                                    },
                                }
                            },
                            None => panic!("No link found"),
                        }
                   },
                    false => {
                        panic!("url not found: {}", &self.url_index);
                    },
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
        TypeCommandRun::Syl(cmd!(SUDO, FSTABGEN, ARGS_U, LOC_HG_FOQ).pipe(cmd!(SUDO, TEE, LOC_HG_FSTAB)))
    }
}
