use std::path::Path;
use std::process::Command;

use const_format::formatcp;
use itertools::{Itertools, Position};

use crate::app::commands::list::ListFromCommand;

use crate::shared::constants::command::*;
use crate::shared::constants::char::{
    CHAR_X, SEMI_COLON
};

use crate::shared::constants::install::*;

// General commands
const ARTIX_CHROOT: &str = "artix-chroot";
const TAR: &str = "tar";
const BTRFS: &str = "btrfs";
const CAT: &str = "cat";
const CLEAR: &str = "clear";
const CURL: &str = "curl";
const DD: &str = "dd";
const ECHO: &str = "echo";
const FSTABGEN: &str = "fstabgen";
const GROUPMOD: &str = "groupmod";
const INSTALL: &str = "install";
const LN: &str = "ln";
const LOCALE_GEN: &str = "locale-gen";
const MKDIR: &str = "mkdir";
const MKFS_BTRFS: &str = "mkfs.btrfs";
const MKFS_VFAT: &str = "mkfs.vfat";
const MKLABEL: &str = "mklabel";
const MKPART: &str = "mkpart";
const MOUNT: &str = "mount";
const PACMAN: &str = "pacman";
// const PACMAN_KEY: &str = "pacman-key";
const PARTED: &str = "parted";
const PARTPROBE: &str = "partprobe";
const RM: &str = "rm";
const SED: &str = "sed";
const SETUP_KEYMAP: &str = "setup-keymap";
const SYSLINUX_INSTALL_UPDATE: &str = "syslinux-install_update";
const TEE: &str = "tee";
const TRUE: &str = "true";
const UMOUNT: &str = "umount";
const USERMOD: &str = "usermod";
const WGET: &str = "wget";

// general arguments
const ARG_A: &str = "-a";
const ARG_D: &str = "-d";
const ARG_E: &str = "-e";
const ARG_F: &str = "-f";
const ARG_I: &str = "-i";
const ARG_L: &str = "-l";
const ARG_M: &str = "-m";
const ARG_MOD600: &str = "-m600";
const ARG_MOD644: &str = "-m644";
const ARG_MOD700: &str = "-m700";
const ARG_MOD755: &str = "-m755";
const ARG_N: &str = "-n";
const ARG_O: &str = "-o";
const ARG_P: &str = "-p";
const ARG_Q: &str = "-q";
const ARG_R: &str = "-r";
const ARG_S: &str = "-s";
const ARGS_C: &str = "-C";
const ARGS_G: &str = "-G";
const ARGS_L: &str = "-L";
const ARGS_S: &str = "-S";
const ARGS_U: &str = "-U";
const ARGS_SYYU: &str = "-Syyu";

// long arguments
// const ARL_INIT: &str = "--init";
const ARL_DIR: &str = "--directory";
const ARL_GROUP: &str = "--group=";
const ARL_NOCONFIRM: &str = "--noconfirm";
// const ARL_POPULATE: &str = "--populate";
const ARL_OWNER: &str = "--owner=";

// command specific arguments
// const ARTIX_ARM: &str = "artixarm";
const BS_1M: &str = "bs=1M";
const COUNT_32: &str = "count=32";
const EOF: &str = "EOF";
const GPT: &str = "gpt";
const C_PERCENT: &str = "100%";
const IF_DEV_ZERO: &str = "if=/dev/zero";
const FILE_LITERAL: &str = "<<";
const MAIN_VOL_COMPRESS: &str = "compress=zstd";
const OPTIMAL: &str = "optimal";
const ROOT: &str = "root";
const SINGLE: &str = "single";
const SIZE: &str = "size";
const START: &str = "start";
const SUB_VOL_COMPRESS: &str = formatcp!("{MAIN_VOL_COMPRESS},subvol=");

// reg expressions
const REX_HD: &str = "'s/\\s*\\([\\+0-9a-zA-Z]*\\).*/\\1/'";

// cleanup dirs
pub const CLEANUP_DIRS: [&str; 10] = [
    formatcp!("{UMOUNT} {DIR_MNT}/root/var/cache/pacman/pkg"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/usr/bin/qemu-aarch64-static"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/var/cache/packman/pkg/*"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/var/log/*"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/etc/*.pacnew"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/usr/lib/systemd/system/systemd-firstboot.service"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/root/etc/machine-id"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/user"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/password"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/rootpassword"),
];

// sh arguments
const ASC_BOOT_SPACE: &str = "32M 512M";
const ASC_BTRFS_SUCR: &str = formatcp!("{BTRFS} su cr");
const ASC_CAT_SB: &str = formatcp!("{CAT} {DIR_SYS_BLOCK}");
const ASC_END_SECTOR: &str = "\"${END_SECTOR}s\"";
const ASC_MKDIR_P: &str = formatcp!("{MKDIR} {ARG_P}");
const ASC_MKLABEL_GPT: &str = formatcp!("{MKLABEL} {GPT}");
const ASC_MKPART_PRIMARY: &str = formatcp!("{MKPART} {PRIMARY}");
const ASC_MOUNT_O: &str = formatcp!("{MOUNT} {ARG_O}");
// const ASC_PACMAN_KEY_INIT: &str = formatcp!("{PACMAN_KEY} {ARL_INIT}");
// const ASC_PACMAN_KEY_POPULATE: &str = formatcp!("{PACMAN_KEY} {ARL_POPULATE} {ARTIX_ARM}");
const ASC_PARTED_OPT: &str = formatcp!("{PARTED} {ARG_A} {OPTIMAL} {ARG_S}");
const ASC_PARTED_S: &str = formatcp!("{PARTED} {ARG_S}");
const ASC_TAR: &str = formatcp!("{TAR} {CHAR_X} {ARG_F}");

// sh hg arguments
const AHG_BOOTLOADER_INSTALL: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {SYSLINUX_INSTALL_UPDATE} {ARG_I} {ARG_A} {ARG_M}");
const AHG_PACMAN_INSTALL: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {PACMAN} {ARGS_S} {ARL_NOCONFIRM}");
const AHG_PACMAN_UPDATE: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {PACMAN} {ARGS_SYYU} {ARL_NOCONFIRM}");
const AHG_FSTABGEN: &str = formatcp!("{FSTABGEN} {ARGS_U} {DIR_HG_ROOT} | {TEE} {ARG_A} {LOC_FSTAB}");
const AHG_RM_STS: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {RM} {LOC_MKINITCPIO_STS}");

// sh multiline arguments
const ASML_LUNARVIM: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {SED} {ARG_E} {REX_HD} {ASC_QUIET} {FILE_LITERAL} {EOF}\n \
        | LV_BRANCH='lelease-1.3/neovim-0.9'\n \
        {CURL} {ARG_S} {DEFAULT_URL_EDITOR}\n \
        | bash\n \
        n\n \
        n\n \
        y\n \
        {EOF}");
// const ASML_PACMAN_KEY: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {ASC_PACMAN_KEY_INIT}; \
//            {ARTIX_CHROOT} {DIR_HG_ROOT} {ASC_PACMAN_KEY_POPULATE}");
const ASML_EQSTALX_FS: &str = formatcp!("{AHG_RM_STS};{AHG_PACMAN_INSTALL} {DEFAULT_PACKAGE_FS}");
const ASC_INSTALL_QEMU_STATIC: &str = formatcp!("{INSTALL} {ARG_MOD755} {ARGS_C} {LOC_QEMU_USER_STATIC} {LOC_HG_QEMU_USER_STATIC}; \
        {CAT} {LOC_BINFMT_AARCH64} | {TEE} {LOC_BINFMT_REGISTER}\
");

#[derive(Clone, Copy)]
pub struct CommandAction {}
    
impl CommandAction {

    pub fn azjx_editor() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(ASML_LUNARVIM);
        Some(cmd)
    }

    pub fn azjx_rezosur() -> Option<Command> {
        Some(Command::new(TRUE))
    }

    pub fn eqstalx_bootloader() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C).arg(AHG_BOOTLOADER_INSTALL);
        Some(cmd)
    }
 
    pub fn eqstalx_packages(packages: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C).arg(format!("{AHG_PACMAN_INSTALL} {packages}"));
        Some(cmd)
    }
    
    pub fn bridge_arch_gap() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C);
        cmd.arg(ASC_INSTALL_QEMU_STATIC);
        Some(cmd)
    }

    pub fn clean_up_install(arch: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C);
        let mut command_sh = String::new(); 
        for dir in CLEANUP_DIRS {
            command_sh.push_str(dir); 
            command_sh.push(SEMI_COLON);
        }
        command_sh.push_str(format!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/Manjaro-ARM-{arch}-latest.tar.gz*").as_str());
        cmd.arg(command_sh);
        Some(cmd)
    }

    pub fn clear() -> Option<Command> {
        Some(Command::new(CLEAR))
    }
 
    pub fn dd_first_mbs(path_drive: &Path) -> Option<Command> {
        let dis_drive = path_drive.display();
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{DD} {IF_DEV_ZERO} of={dis_drive} {BS_1M} {COUNT_32} {ASC_QUIET}"));
        Some(cmd)
    }

    pub fn extract_rootfs(loc_file: &str, dir_dest: &str) -> Option<Command> {
        let mut dir_end = String::from(dir_dest);
        dir_end.push_str(DIR_VAR_TMP);
        match Path::new(&dir_end).exists() {
            false => {
                let mut cmd = Command::new(SUDO);
                cmd.args(ARG_SH_C)
                .arg(format!("{ASC_TAR} {loc_file} {ARGS_C} {dir_dest}"));
                Some(cmd)
            },
            true => None,
        }
    }

    pub fn update_packages() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C).arg(AHG_PACMAN_UPDATE);
        Some(cmd)
    }

    pub fn make_dirs(dirs: &[&str]) -> Option<Command> {
        let dirs: Vec<&Path> = dirs.iter().filter_map(|dir| {
            let dir = Path::new(dir);
            match dir.exists() {
                    true => None,
                    false => Some(dir)
            }
        }).collect::<Vec<&Path>>();
        
        match dirs.is_empty() {
            true => None,
            false => {
                let mut cmd = Command::new(SUDO);
                cmd.args(ARG_SH_C).
                    arg(dirs.iter().with_position().map(|e| {
                    match e {
                        (Position::First, dir) | (Position::Middle, dir) => format!("{ASC_MKDIR_P} {}; ", dir.display()),
                        (_, dir) => format!("{ASC_MKDIR_P} {}", dir.display()),
                    }
                }).collect::<String>());
                Some(cmd)
            },
        }
    }

    pub fn make_label(drive: &Path) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_PARTED_S} {} {ASC_MKLABEL_GPT} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn make_boot_partition(drive: &Path, partition_type: &str) -> Option<Command> { 
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {partition_type} {ASC_BOOT_SPACE} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn make_root_partition(drive: &Path, partition_type: &str, partition_prev: u32) -> Option<Command> {
        match drive.file_name().unwrap().to_str() {
            Some(device) => {
                let calc = format!("START=`{ASC_CAT_SB}/{device}/{device}{partition_prev}/{START}`; \
                    SIZE=`{ASC_CAT_SB}/{device}/{device}{partition_prev}/{SIZE}`; \
                    END_SECTOR=$(expr $START + $SIZE);");

                let mut cmd = Command::new(SUDO);
                cmd.args(ARG_SH_C)
                .arg(format!("{} {ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_END_SECTOR} {C_PERCENT} {ASC_QUIET}", 
                    calc, drive.display(), partition_type));
                info!("calc cmd:{:?}", cmd);
                Some(cmd)
            },
            None => panic!("Cannot unwrap device name")
        }

    }

    pub fn make_subvol(drive: &Path) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_BTRFS_SUCR} {} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mkfs_btrfs(drive: &Path, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{MKFS_BTRFS} {ARG_M} {SINGLE} {ARGS_L} {LABEL_ROOT_AND_HOME} {ARG_F} {}{partition} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mkfs_vfat(drive: &Path, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C); 
        cmd.arg(format!("{MKFS_VFAT} {ARG_N} {LABEL_BOOT} {}{partition} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mount(drive: &Path, dir: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.arg(MOUNT)
        .arg(format!("{}", drive.display()))
        .arg(dir);
        Some(cmd)
    }

    pub fn mount_mainvol(partition: &Path, dir: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_MOUNT_O} {MAIN_VOL_COMPRESS} {} {}", 
            partition.display(), dir));
        Some(cmd)
    }

    pub fn mount_subvols(partition: &Path, subvols: &[(&str, &str)]) -> Option<Command> {
        let mut command_sh = String::new();
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C);
        
        let mut subvols = subvols.iter().with_position().peekable();
        while let Some(e) = subvols.next() {
            match e {
                (Position::First, subvol) | (Position::Middle, subvol) => 
                    command_sh.push_str(format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}; {ASC_MKDIR_P} {}; ", 
                        subvol.0, partition.display(), subvol.1, subvols.peek().unwrap().1.1).as_str()),
                (_, subvol) => command_sh.push_str(format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}", subvol.0, partition.display(), subvol.1).as_str()),
            }
        }
        cmd.arg(command_sh);
        Some(cmd)
    }


    pub fn partprobe(drive: &Path) -> Option<Command> {
        let command_sh = format!("{} {} {}", PARTPROBE, drive.display(), ASC_QUIET);
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
            .arg(command_sh.clone());
        Some(cmd)
    }

    pub fn remove_partitions(drive: &Path) -> Option<Command> {
        let dis_drive = drive.display();
        let list_pts = ListFromCommand::all_partition_numbers(drive);

        match list_pts.len() {
            0 => None,
             _ => {
                let mut cmd = Command::new(SUDO); 
                cmd.args(ARG_SH_C);
                let sh_remove: String = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition} {ASC_QUIET}; "),
                        (_, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition} {ASC_QUIET}"),
                    }
                }).collect();
                match Self::umount_drive(drive) {
                    Some(sh_umount) => cmd.arg(format!("{}{}", sh_umount, sh_remove)),
                    None => cmd.arg(sh_remove),
                };
                Some(cmd)        
            }
        }
    }

    pub fn eqstalx_fs() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C).arg(ASML_EQSTALX_FS);
        Some(cmd)
    }

    pub fn set_settings_system(region_timezone: &str, zone_timezone: &str, locale: &str, keymap: &str, name_host: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ARTIX_CHROOT} {DIR_HG_ROOT} {LN} {ARG_S} {ARG_F} /usr/share/zoneinfo/timezone/{region_timezone}/{zone_timezone} /etc/localtime; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {SED} {ARG_I} s/\"#{locale}\"/\"{locale}\"/g {LOC_LOCALE_GEN}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {ECHO} \"LOCALE={locale}\" | {TEE} {ARG_A} {LOC_LOCALE_CONF}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {LOCALE_GEN} \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {ECHO} \"KEYMAP={keymap}\nFONT={DEFAULT_CONSOLEFONT}\" | {TEE} {ARG_A} {LOC_VCONSOLE_CONF}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {LOCALE_GEN} \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {name_host} | {TEE} {ARG_A} {LOC_HOSTNAME} \
        "));
        Some(cmd)
    }

    pub fn set_users(user: &str, name_full: &str, password_user: &str, password_root: &str, key_pub_user: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(format!("{ARTIX_CHROOT} {DIR_HG_ROOT} {GROUPMOD} {ARG_N} {user} {DEFAULT_USERGROUP_USER}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {USERMOD} {ARG_A} {ARGS_G} \
        {DEFAULT_USERGROUPS} {ARG_P} {password_user} {ARG_S} {DEFAULT_SHELL} \
        {ARG_L} {user} {ARG_C} {name_full} {ARG_M} {ARG_D} /home/{user} {DEFAULT_USERNAME}; \
        {INSTALL} {ARL_DIR} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD700} {DIR_HG_ROOT}/home/{user}/.ssh; \
        {INSTALL} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD600} {DIR_HG_ROOT}/home/{user}/authorized_keys; \
        {ECHO} {DIR_HG_ROOT}/.ssh/{key_pub_user} | {TEE} {ARG_A} {DIR_HG_ROOT}/home/{user}/authorized_keys {ASC_QUIET}; \
        {INSTALL} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD644} {LOC_PROFILE} {DIR_HG_ROOT}/home/{user}/.profile; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {USERMOD} {ARG_P} {password_root} {ROOT} \
        "));
        Some(cmd)
    }

    pub fn setup_keymap(keymap: &str, keyvar: &str) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.arg(SETUP_KEYMAP).arg(keymap).arg(keyvar);
        Some(cmd)
    }

    /*
    pub fn setup_keyrings() -> Option<Command> { 
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C)
        .arg(ASML_PACMAN_KEY);
        Some(cmd)
    }
    */

    pub fn _show_elapsed_time() -> Option<Command> {
        Some(Command::new(TRUE))
    }
    
    pub fn umount(path: &Path) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.arg(UMOUNT)
        .arg(format!("{}", path.display()));
        Some(cmd)
    }

    fn umount_drive(drive: &Path) -> Option<String> { 
        let list_pts = ListFromCommand::mounted_partitions(drive);

        match list_pts.len() {
            0 => None,
            _ => {
                Some(list_pts.iter().map(|partition| format!("{UMOUNT} {partition} {ASC_QUIET}; ")).collect())
            }
        }
    }

    pub fn umount_dirs(dirs: &[&str]) -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C);
       
        let sh_command: String = dirs.iter().with_position().map(|e| {
            match e {
                (Position::First, dir) | (Position::Middle, dir) => format!("{UMOUNT} {dir}; "),
                (_, dir) => format!("{UMOUNT} {dir}"),
            }
        }).collect();

        cmd.arg(sh_command);
        Some(cmd)
    }

    pub fn wget(dir_end: &str, url_download: &str) -> Option<Command> {
        match Path::new(&format!("{dir_end}/{FILE_XZ_ARMTIX}")).exists() {
            true => None,
            false => {
                let mut cmd = Command::new(SUDO);
                cmd.current_dir(dir_end);
                cmd.args([WGET, ARG_Q, url_download]);
                Some(cmd)
            },
        }
    }

    pub fn zjenx_fstab() -> Option<Command> {
        let mut cmd = Command::new(SUDO);
        cmd.args(ARG_SH_C).arg(AHG_FSTABGEN);
        Some(cmd)
    }
}
