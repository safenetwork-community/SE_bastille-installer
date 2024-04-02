use std::path::PathBuf;
use std::process::Command;

use const_format::formatcp;
use itertools::{Itertools, Position};

use crate::app::commands::list::ListFromCommand;

use crate::shared::constants::command::*;
use crate::shared::constants::char::SEMI_COLON;
use crate::shared::constants::string::JOIN_SPACE;

use crate::shared::constants::install::{
    DIR_TMP,
    DIR_PKG_CACHE,
    DIR_PAC_PKG,
    TYPE_PART_PRIMARY,
};

// General commands
const ARTIX_CHROOT: &str = "artix-chroot";
const BSDTAR: &str = "bsdtar";
const CLEAR: &str = "clear";
const DD: &str = "dd";
const MKDIR: &str = "mkdir";
const MKFS_BTRFS: &str = "mkfs.btrfs";
const MKFS_VFAT: &str = "mkfs.vfat";
const MKLABEL: &str = "mklabel";
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
const ARG_F: &str = "-f";
const ARG_F10: &str = "-f10";
const ARG_M: &str = "-m";
const ARG_N: &str = "-n";
const ARG_O: &str = "-o";
const ARG_P: &str = "-p";
const ARG_R: &str = "-r";
const ARG_S: &str = "-s";
const ARG_X: &str = "-x";
const ARGS_G: &str = "-G";
const ARGS_L: &str = "-L";
const ARGS_SYYU: &str = "-Syyu";
const ARL_INIT: &str = "--init";
const ARL_NOCONFIRM: &str = "--noconfirm";
const ARL_POPULATE: &str = "--populate";
const ONE_G: &str = "1>";
const TWO_GN_ONE: &str = "2>&1";

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
const LABEL_BOOT: &str = "BOOT_BASTIJ";
const LABEL_ROOT: &str = "ROOT_BASTIJ";
const PACMAN_MIRRORS: &str = "pacman-mirrors";
const SINGLE: &str = "single";
const SIZE: &str = "size";
const START: &str = "start";
const SUB_VOL_COMPRESS: &str = "compress=zstd,subvol=";
const UBOOT_RASPBERRY_PI: &str = "uboot-raspberrpi";
const WHEEL: &str = "wheel";
const IF_DEV_ZERO: &str = "if=/dev/zero";

// argument character strings
const AND: &str = "&";
const PLUS: &str = "+";
const SLASH: &str = "/";

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
const DEV_NULL: &str = "/dev/null";
const SYS_BLOCK: &str = "/sys/block";

// general error messages
const ERR_FAIL_UNWRAP_OUTPUT_COMMAND: &str = "Failed to unwrap output command";
const ERR_FAIL_EXECUTE_PROCESS: &str = "Process failed to execute";

// arguments
const QUIET: &str = formatcp!("{ONE_G} {DEV_NULL} {TWO_GN_ONE}");
const ARG_PACMAN_KEY_INIT: &str = formatcp!("{PACMAN_KEY} {ARL_INIT} {QUIET}");
const ARG_PACMAN_KEY_POPULATE: &str = formatcp!("{PACMAN_KEY} {ARL_POPULATE} {ARTIX_ARM} {QUIET}");
const ARG_PARTED_S: &str = formatcp!("{PARTED} {ARG_S}");

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
        cmd.args([ARTIX_CHROOT, dir, USERADD, ARG_M, ARGS_G, WHEEL, ARG_P, password_user, AND]);
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
 
    pub fn dd_first_mbs(pathbuf_drive: PathBuf) -> Option<Command> { 
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{} {} of={} {} {} {}", 
        DD, IF_DEV_ZERO, pathbuf_drive.display(), BS_1M, COUNT_32, QUIET));
        Some(cmd)
    }
 
    pub fn enable_services_root(dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args([ARTIX_CHROOT, dir, DINIT, ENABLE, GETTY_TARGET, ONE_G, DEV_NULL]);
        Some(cmd)
    }

    pub fn enable_services_user() -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(["echo"]);
        Some(cmd)
    }

    pub fn extract_rootfs(file_loc: &str, dest_dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args([BSDTAR, ARG_F, ARG_P, ARG_X, file_loc, dest_dir]);
        Some(cmd)
    }


    pub fn install_packages(dir: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        let command_sh = format!(
            r#"
            {MKDIR} {ARG_P} {DIR_PKG_CACHE}; \ 
            {MKDIR} {ARG_O} {DIR_PKG_CACHE} {DIR_PAC_PKG}; \
            {ARTIX_CHROOT} {dir} {PACMAN} {ARGS_SYYU} \
            {BASE} {UBOOT_RASPBERRY_PI} {CONMAN_DINIT} \
            {ARL_NOCONFIRM}
        "#);
        cmd.arg(command_sh);
        Some(cmd)
    }

    pub fn make_dirs(dirs: &[&str]) -> Option<Command> {
        let mut cmd = Command::new(MKDIR);
        cmd.args(ARG_SH_C);
        let sh_command: String = dirs.iter().with_position().map(|e| {
            match e {
                (Position::First, dir) | (Position::Middle, dir) => format!("{MKDIR} {dir}; "),
                (_, dir) => format!("{MKDIR} {dir}"),
            }
        }).collect();
        cmd.arg(sh_command);
        Some(cmd)
    }

    pub fn make_label(drive: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{} {} {} {} {} {}", 
            PARTED, ARG_S, drive, 
            MKLABEL, GPT, QUIET
        ));
        Some(cmd)
    }

    pub fn make_boot_partition(drive: &str, partition_type: &str) -> Option<Command> { 
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{} {} {} {} {} {} {} {}",
            PARTED, ARG_S, drive, 
            TYPE_PART_PRIMARY, partition_type, 
            "32M", "512M", QUIET
        ));
        Some(cmd)
    }

    pub fn make_root_partition(drive: &str, partition_type: &str, partition_prev: u32) -> Option<Command> {
        let start = Self::get_output_command(Command::new("cat")
            .arg(format!("{SYS_BLOCK}{SLASH}{drive}{SLASH}{drive}{partition_prev}{SLASH}{START}")));
        let size = Self::get_output_command(Command::new("cat")
            .arg(format!("{SYS_BLOCK}{SLASH}{drive}{SLASH}{drive}{SLASH}{SIZE}"))); 
        let end_sector = Self::get_output_command(Command::new("expr")
            .args([start, PLUS.to_string(), size]));

        let mut cmd = Command::new(PARTED);
        cmd.args([ARG_S, drive, TYPE_PART_PRIMARY, partition_type, end_sector.as_str(), "100%", QUIET]);
        Some(cmd)
    }

    pub fn mkfs_btrfs(drive: &str, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        cmd.arg(format!("{MKFS_BTRFS} {ARG_M} {SINGLE} {ARGS_L} {LABEL_ROOT} {ARG_F} {drive}{partition} {QUIET}"));
        Some(cmd)
    }

    pub fn mkfs_vfat(drive: &str, partition: u32) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C); 
        cmd.arg(format!("{MKFS_VFAT} {ARG_N} {LABEL_BOOT} {drive}{partition} {QUIET}"));
        Some(cmd)
    }

    pub fn mount(dir: &str, drive: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(MOUNT)
        .arg(drive)
        .arg(dir);
        Some(cmd)
    }

    pub fn mount_subvols(dir: &str, subvols: Vec<&str>) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(MOUNT)
        .args(subvols)
        .arg(dir);
        Some(cmd)
    }


    pub fn partprobe(drive: &str) -> Option<Command> {
        let mut cmd = Command::new(PARTPROBE);
        cmd.args([drive, QUIET]);
        Some(cmd)
    }

    pub fn remove_partitions(drive: &str) -> Option<Command> {
        let list_pts = ListFromCommand::mounted_partitions(drive);

        match list_pts.len() {
            0 => None,
             _ => {
                let mut cmd = Command::new(DOAS); 
                cmd.args(ARG_SH_C);
                let sh_command: String = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{ARG_PARTED_S} {drive} {RM} {partition} {QUIET}; "),
                        (_, partition) => format!("{ARG_PARTED_S} {drive} {RM} {partition} {QUIET}"),
                    }
                }).collect();
                cmd.arg(sh_command);
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
        cmd.args([ARTIX_CHROOT, dir, PACMAN_MIRRORS, ARG_F10, QUIET]);
        Some(cmd)
    }

    pub fn setup_keyrings(dir: &str) -> Option<Command> { 
        let mut cmd = Command::new(DOAS);
        cmd.args(ARG_SH_C);
        let mut command_sh = String::new();
        command_sh.push_str(&[ARTIX_CHROOT, dir, ARG_PACMAN_KEY_INIT].join(JOIN_SPACE));
        command_sh.push(SEMI_COLON);
        command_sh.push_str(&[ARTIX_CHROOT, dir, ARG_PACMAN_KEY_POPULATE].join(JOIN_SPACE));
        Some(cmd)
    }

    pub fn make_subvols(drive: &str, partition: u32, dir: &str, subvolume: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.args([MOUNT, ARG_O, format!("{SUB_VOL_COMPRESS}{subvolume}").as_str(), format!("{drive}{partition}").as_str(), dir]);
        Some(cmd)
    }

    pub fn _show_elapsed_time() -> Option<Command> {
        let mut cmd = Command::new("sleep");
        cmd.arg("0.1");
        Some(cmd)
    }

    pub fn umount_drive(drive: &str) -> Option<Command> {
        let list_pts = ListFromCommand::mounted_partitions(drive);

        // info!("drive: {drive}");
        // info!("partitions: {:?}", list_pts);

        match list_pts.len() {
            0 => None,
            _ => {
                let mut cmd = Command::new(DOAS); 
                cmd.args(ARG_SH_C);
                let sh_command: String = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{UMOUNT} {partition} {QUIET}; "),
                        (_, partition) => format!("{UMOUNT} {partition} {QUIET}"),
                    }
                }).collect();
                cmd.arg(sh_command);
                Some(cmd)        
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
        let mut cmd = Command::new(WGET);
        cmd.current_dir(current_dir);
        cmd.args(["-q", "--show-progress", "--progress=bar:force:noscroll", path]);
        Some(cmd)
    }

    pub fn setup_keymap(keymap: &str, keyvar: &str) -> Option<Command> {
        let mut cmd = Command::new(DOAS);
        cmd.arg(SETUP_KEYMAP)
        .arg(keymap)
        .arg(keyvar);
        Some(cmd)
    }

    fn get_output_command(command: &mut Command) -> String {
        let output = command
            .output()
            .unwrap_or_else(|_| panic!("{}:\n{:?}\n{:?}", ERR_FAIL_EXECUTE_PROCESS, command.get_args(), command.get_program()));
        String::from_utf8(output.stdout).expect(ERR_FAIL_UNWRAP_OUTPUT_COMMAND)
    }
}
