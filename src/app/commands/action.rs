use std::path::Path;
use std::process::Command;

use const_format::formatcp;
use itertools::{Itertools, Position};

use crate::app::commands::list::ListFromCommand;

use crate::shared::constants::command::*;
use crate::shared::constants::char::{
    CHAR_X, SEMI_COLON
};

use crate::shared::constants::install::{
    DIR_TMP,
    DIR_PKG_CACHE,
    DIR_PAC_PKG,
    PRIMARY,
};

// General commands
const ARTIX_CHROOT: &str = "artix-chroot";
const TAR: &str = "tar";
const BTRFS: &str = "btrfs";
const CAT: &str = "cat";
const CLEAR: &str = "clear";
const DD: &str = "dd";
const MKDIR: &str = "mkdir";
const MKFS_BTRFS: &str = "mkfs.btrfs";
const MKFS_VFAT: &str = "mkfs.vfat";
const MKLABEL: &str = "mklabel";
const MKPART: &str = "mkpart";
const MOUNT: &str = "mount";
const PACMAN: &str = "pacman";
const PACMAN_KEY: &str = "pacman-key";
const PARTED: &str = "parted";
const PARTPROBE: &str = "partprobe";
const RM: &str = "rm";
const SETUP_KEYMAP: &str = "setup-keymap";
const UMOUNT: &str = "umount";
const USERADD: &str = "useradd";
const WGET: &str = "wget";

// general arguments
const ARG_A: &str = "-a";
const ARG_F: &str = "-f";
const ARG_F10: &str = "-f10";
const ARG_M: &str = "-m";
const ARG_N: &str = "-n";
const ARG_O: &str = "-o";
const ARG_P: &str = "-p";
const ARG_Q: &str = "-q";
const ARG_R: &str = "-r";
const ARG_S: &str = "-s";
const ARGS_C: &str = "-C";
const ARGS_G: &str = "-G";
const ARGS_L: &str = "-L";
const ARGS_SYYU: &str = "-Syyu";
const ARL_INIT: &str = "--init";
const ARL_NOCONFIRM: &str = "--noconfirm";
const ARL_POPULATE: &str = "--populate";

// command specific arguments
const ARTIX_ARM: &str = "artixarm";
const BASE: &str = "base";
const BS_1M: &str = "bs=1M";
const CONMAN_DINIT: &str = "conman-dinit";
const COUNT_32: &str = "count=32";
const DINIT: &str = "dinit";
const ENABLE: &str = "enable";
const GETTY_TARGET: &str = "getty.target";
const GPT: &str = "gpt";
const C_PERCENT: &str = "100%";
const LABEL_BOOT: &str = "BOOT_BASTIJ";
const LABEL_ROOT: &str = "ROOT_BASTIJ";
const OPTIMAL: &str = "optimal";
const PACMAN_MIRRORS: &str = "pacman-mirrors";
const SINGLE: &str = "single";
const SIZE: &str = "size";
const START: &str = "start";
const STR_AND: &str = "&";
const SUB_VOL_COMPRESS: &str = "compress=zstd,subvol=";
const UBOOT_RASPBERRY_PI: &str = "uboot-raspberrpi";
const WHEEL: &str = "wheel";
const IF_DEV_ZERO: &str = "if=/dev/zero";

// argument character strings

// dirs
const CLEANUP_DIRS: [&str; 10] = [
    formatcp!("{UMOUNT} {DIR_TMP}/root/var/cache/pacman/pkg"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/usr/bin/qemu-aarch64-static"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/var/cache/packman/pkg/*"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/var/log/*"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/etc/*.pacnew"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/usr/lib/systemd/system/systemd-firstboot.service"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/root/etc/machine-id"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/user"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/password"),
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/rootpassword"),
];
const SYS_BLOCK: &str = "/sys/block";
const DIR_VAR_TMP: &str = "/var/tmp";

// general sh arguments
// const ASC_QUIET: &str = formatcp!("{ONE_G} {DEV_NULL} {TWO_GN_ONE}");
const ASC_QUIET: &str = formatcp!("");

// sh arguments
const ASC_BOOT_SPACE: &str = "32M 512M";
const ASC_BTRFS_SUCR: &str = formatcp!("{BTRFS} su cr");
const ASC_CAT_SB: &str = formatcp!("{CAT} {SYS_BLOCK}");
const ASC_END_SECTOR: &str = "\"${END_SECTOR}s\"";
const ASC_MKDIR_P: &str = formatcp!("{MKDIR} {ARG_P}");
const ASC_MKLABEL_GPT: &str = formatcp!("{MKLABEL} {GPT}");
const ASC_MKPART_PRIMARY: &str = formatcp!("{MKPART} {PRIMARY}");
const ASC_MOUNT_O: &str = formatcp!("{MOUNT} {ARG_O}");
const ASC_PACMAN_KEY_INIT: &str = formatcp!("{PACMAN_KEY} {ARL_INIT} {ASC_QUIET}");
const ASC_PACMAN_KEY_POPULATE: &str = formatcp!("{PACMAN_KEY} {ARL_POPULATE} {ARTIX_ARM} {ASC_QUIET}");
const ASC_PARTED_OPT: &str = formatcp!("{PARTED} {ARG_A} {OPTIMAL} {ARG_S}");
const ASC_PARTED_S: &str = formatcp!("{PARTED} {ARG_S}");
const ASC_TAR: &str = formatcp!("{TAR} {CHAR_X} {ARG_F}");

// slice arguments 
const ASCS_DINIT_E: [&str; 3] = [DINIT, ENABLE, GETTY_TARGET];


#[derive(Clone, Copy)]
pub struct CommandAction {}
    
impl CommandAction {
/*
    pub fn add_bootloader() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg("echo");
        Some(cmd)
    }
*/
    pub fn apply_overlay() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg("echo");
        Some(cmd)
    }


    pub fn add_support_btrfs() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(["echo"]);
        Some(cmd)
    }

    pub fn add_users(dir: &str, password_user: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args([ARTIX_CHROOT, dir, USERADD, ARG_M, ARGS_G, WHEEL, ARG_P, password_user])
        .arg(STR_AND);
        Some(cmd)
    }

    pub fn clean_up_install(arch: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        let mut command_sh = String::new(); 
        for dir in CLEANUP_DIRS {
            command_sh.push_str(dir); 
            command_sh.push(SEMI_COLON);
        }
        command_sh.push_str(format!("{RM} {ARG_R} {ARG_F} {DIR_TMP}/Manjaro-ARM-{arch}-latest.tar.gz*").as_str());
        cmd.arg(command_sh);
        Some(cmd)
    }

    pub fn clear() -> Option<Command> {
        Some(Command::new(CLEAR))
    }
 
    pub fn dd_first_mbs(path_drive: &Path) -> Option<Command> {
        let dis_drive = path_drive.display();
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{DD} {IF_DEV_ZERO} of={dis_drive} {BS_1M} {COUNT_32} {ASC_QUIET}"));
        Some(cmd)
    }
 
    pub fn enable_services_root(dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(ARTIX_CHROOT)
        .arg(dir)
        .args(ASCS_DINIT_E)
        .args(ASCS_QUIET);
        Some(cmd)
    }

    pub fn enable_services_user() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(["echo"]);
        Some(cmd)
    }

    pub fn extract_rootfs(file_loc: &str, dest_dir: &str) -> Option<Command> {
        let mut end_dir = String::from(dest_dir);
        end_dir.push_str(DIR_VAR_TMP);
        match Path::new(&end_dir).exists() {
            false => {
                let mut cmd = Command::new(DOAS);
                cmd.args(ARG_SH_C)
                .arg(format!("{ASC_TAR} {file_loc} {ARGS_C} {dest_dir}"));
                Some(cmd)
            },
            true => None,
        }
    }


    pub fn install_packages(dir: &str) -> Option<Command> {
        let dir: &Path = Path::new(dir);
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        let command_sh = format!(
            r#"
            {MKDIR} {ARG_P} {DIR_PKG_CACHE}; \ 
            {MKDIR} {ARG_O} {DIR_PKG_CACHE} {DIR_PAC_PKG}; \
            {ARTIX_CHROOT} {} {PACMAN} {ARGS_SYYU} \
            {BASE} {UBOOT_RASPBERRY_PI} {CONMAN_DINIT} \
            {ARL_NOCONFIRM}
        "#, dir.display());
        cmd.arg(command_sh);
        Some(cmd)
    }

    /*
    pub fn make_dir(dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_MKDIR_P} {}", 
            Path::new(dir).display()));
        Some(cmd)
    }
    */
    
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
                let mut cmd = Command::new(DOAS);
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
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("TEST=$(whoami); echo $TEST;{} {} {} {}", ASC_PARTED_S, drive.display(), ASC_MKLABEL_GPT, ASC_QUIET));
        Some(cmd)
    }

    pub fn make_boot_partition(drive: &Path, partition_type: &str) -> Option<Command> { 
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{} {} {} {} {} {}", ASC_PARTED_OPT, drive.display(), ASC_MKPART_PRIMARY, 
            partition_type, ASC_BOOT_SPACE, ASC_QUIET));
        Some(cmd)
    }

    pub fn make_root_partition(drive: &Path, partition_type: &str, partition_prev: u32) -> Option<Command> {
        match drive.file_name().unwrap().to_str() {
            Some(device) => {
                let calc = format!("START=`{ASC_CAT_SB}/{device}/{device}{partition_prev}/{START}`; \
                    SIZE=`{ASC_CAT_SB}/{device}/{device}{partition_prev}/{SIZE}`; \
                    END_SECTOR=$(expr $START + $SIZE);");

                info!("calc:{}", calc);

                let mut cmd = Command::new(DOAS);
                cmd.args(ARG_SH_C)
                .arg(format!("{} {ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_END_SECTOR} {C_PERCENT} {ASC_QUIET}", 
                    calc, drive.display(), partition_type));
                Some(cmd)
            },
            None => panic!("Cannot unwrap device name")
        }

    }

    pub fn make_subvol(drive: &Path) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_BTRFS_SUCR} {} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mkfs_btrfs(drive: &Path, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{MKFS_BTRFS} {ARG_M} {SINGLE} {ARGS_L} {LABEL_ROOT} {ARG_F} {}{partition} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mkfs_vfat(drive: &Path, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C); 
        cmd.arg(format!("{MKFS_VFAT} {ARG_N} {LABEL_BOOT} {}{partition} {ASC_QUIET}", drive.display()));
        Some(cmd)
    }

    pub fn mount(drive: &Path, dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(MOUNT)
        .arg(format!("{}", drive.display()))
        .arg(dir);
        Some(cmd)
    }

    pub fn mount_mainvol(partition: &Path, dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS} {} {}", 
            partition.display(), dir));
        Some(cmd)
    }

    pub fn mount_subvols(partition: &Path, subvols: &[(&str, &str)]) -> Option<Command> {
        let mut command_sh = String::new();
        let mut cmd = Command::new(DOAS);
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
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
            .arg(command_sh.clone());
        Some(cmd)
    }

    pub fn remove_partitions(drive: &Path) -> Option<Command> {
        let dis_drive = drive.display();
        let list_pts = ListFromCommand::mounted_partition_numbers(drive);

        match list_pts.len() {
            0 => None,
             _ => {
                let mut cmd = Command::new(DOAS); 
                cmd.args(ARG_SH_C);
                let sh_command: String = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}{ASC_QUIET}; "),
                        (_, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}{ASC_QUIET}"),
                    }
                }).collect();
                cmd.arg(format!("{}{}", Self::umount_drive(drive).unwrap(), sh_command));
                Some(cmd)        
            }

        }
    }

    pub fn set_settings_system() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(["echo"]);
        Some(cmd)
    }

    pub fn set_list_mirror(dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args([ARTIX_CHROOT, dir, PACMAN_MIRRORS, ARG_F10, ASC_QUIET]);
        Some(cmd)
    }

    pub fn setup_keyrings(dir: &str) -> Option<Command> { 
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C)
        .arg(format!("{ARTIX_CHROOT} {dir} {ASC_PACMAN_KEY_INIT}{SEMI_COLON} \
            {ARTIX_CHROOT} {dir} {ASC_PACMAN_KEY_POPULATE}"));
        Some(cmd)
    }

    pub fn _show_elapsed_time() -> Option<Command> {
        let mut cmd = Command::new("sleep");
        cmd.arg("0.1");
        Some(cmd)
    }
    
    pub fn umount(path: &Path) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(UMOUNT)
        .arg(format!("{}", path.display()));
        Some(cmd)
    }

    fn umount_drive(drive: &Path) -> Option<String> { 
        let list_pts = ListFromCommand::mounted_partitions(drive);
        // info!("partitions umount_drive: {:?}", list_pts);

        match list_pts.len() {
            0 => None,
            _ => {
                Some(list_pts.iter().map(|partition| format!("{UMOUNT} {partition} {ASC_QUIET}; ")).collect())
            }
        }
    }

    pub fn umount_dirs(dirs: &[&str]) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
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

    pub fn wget(current_dir: &str, path: &str) -> Option<Command> {
        match Path::new(path).exists() {
            true => {
                let mut cmd = Command::new(DOAS);
                cmd.current_dir(current_dir);
                cmd.args([WGET, ARG_Q, path]);
                Some(cmd)
            },
            false => None,
        }
    }

    pub fn setup_keymap(keymap: &str, keyvar: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(SETUP_KEYMAP)
        .arg(keymap)
        .arg(keyvar);
        Some(cmd)
    }
}
