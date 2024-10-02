use std::path::{Path, PathBuf};

use const_format::concatcp;
use const_format::formatcp;
use duct::{cmd, Expression};
use itertools::{Itertools, Position};

use crate::app::commands::output::CommandOutput;
use crate::shared::constants::command::*;

use crate::shared::constants::install::*;

// General commands
const BTRFS: &str = "btrfs";
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
const MV: &str = "mv";
const PACMAN: &str = "pacman";
const PARTED: &str = "parted";
const PARTPROBE: &str = "partprobe";
const SED: &str = "sed";
const SYSLINUX_INSTALL_UPDATE: &str = "syslinux-install_update";
const TAR: &str = "tar";
const TEE: &str = "tee";
const TOUCH: &str = "touch";
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
const ARL_DIR: &str = "--directory";
const ARL_GROUP: &str = "--group=";
const ARL_NOCONFIRM: &str = "--noconfirm";
const ARL_OWNER: &str = "--owner=";

// command specific arguments
const BOOT_SPACE: &str = "512M";
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
const STATUS_NONE: &str = "status=none";
const SUB_VOL_COMPRESS: &str = formatcp!("{MAIN_VOL_COMPRESS},subvol=");

// reg expressions
const REX_HD: &str = "'s/\\s*\\([\\+0-9a-zA-Z]*\\).*/\\1/'";

// marker remove commands
const RM_MARKER_KOQSTRUE: &str = concatcp!(COMMA_SPACE, RM, LOC_HG_MAHRK_IMAZJ_KOQSTRUE);
// const RM_MARKER_DATIZJE: &str = concatcp!(COMMA_SPACE, RM, LOC_HG_MAHRK_IMAZJ_DATIZJE);

// marker touch commands
const TOUCH_MARKER_KOQSTRUE: &str = concatcp!(COMMA_SPACE, TOUCH, LOC_HG_MAHRK_IMAZJ_KOQSTRUE);
const TOUCH_MARKER_DATIZJE: &str = concatcp!(COMMA_SPACE, TOUCH, LOC_HG_MAHRK_IMAZJ_DATIZJE);

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
const ASC_BOOT_SPACE: &str = formatcp!("1MiB {BOOT_SPACE}");
const ASC_BTRFS_SUCR: &str = formatcp!("{BTRFS} su cr");
const ASC_DEL_LPT: &str = formatcp!("{BS_1M} {COUNT_32} {STATUS_NONE}");
const ASC_END_SECTOR: &str = formatcp!("{BOOT_SPACE}"); // "\"${END_SECTOR}s\"";
const ASC_MKDIR_P: &str = formatcp!("{MKDIR} {ARG_P}");
const ASC_MKLABEL_GPT: &str = formatcp!("{MKLABEL} {GPT}");
const ASC_MKPART_PRIMARY: &str = formatcp!("{MKPART} {PRIMARY}");
const ASC_MOUNT_O: &str = formatcp!("{MOUNT} {ARG_O}");
const ASC_MV_BOOT: &str = formatcp!("{MV} {DIR_HG_ROOT}/{BOOT}/* {DIR_HG_BOOT}");
// const ASC_PACMAN_KEY_INIT: &str = formatcp!("{PACMAN_KEY} {ARL_INIT}");
// const ASC_PACMAN_KEY_POPULATE: &str = formatcp!("{PACMAN_KEY} {ARL_POPULATE} {ARTIX_ARM}");
const ASC_PARTED_OPT: &str = formatcp!("{PARTED} {ARG_A} {OPTIMAL} {ARG_S}");
const ASC_PARTED_S: &str = formatcp!("{PARTED} {ARG_S}");
const ASC_TAR: &str = formatcp!("{TAR} x {ARG_F}");

// sh hg arguments
const AHG_BOOTLOADER_INSTALL: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {SYSLINUX_INSTALL_UPDATE} {ARG_I} {ARG_A} {ARG_M}");
const AHG_FSTABGEN: &str = formatcp!("{FSTABGEN} {ARGS_U} {DIR_HG_ROOT}");
const AHG_INSTALL_QEMU_STATIC: &str = formatcp!("{INSTALL} {ARG_MOD755} {ARGS_C} {LOC_QEMU_USER_STATIC} {LOC_HG_QEMU_USER_STATIC}");
const AHG_PACMAN_INSTALL: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {PACMAN} {ARGS_S} {ARL_NOCONFIRM}");
const AHG_PACMAN_UPDATE: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {PACMAN} {ARGS_SYYU} {ARL_NOCONFIRM}");
const AHG_REGISTER_QEMU_STATIC: &str = formatcp!("; {CAT} {LOC_BINFMT_AARCH64} | {TEE} {LOC_BINFMT_REGISTER}");
const AHG_RM: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {RM}");
const AHG_RM_STS: &str = formatcp!("{AHG_RM} {LOC_MKINITCPIO_STS}");

// sh multiline arguments
const ASML_LUNARVIM: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {SED} {ARG_E} {REX_HD} {FILE_LITERAL} {EOF}\n \
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

#[derive(Clone, Copy)]
pub struct CommandAction {}
    
impl CommandAction {

    pub fn azjx_editor() -> Option<Expression> { 
        Some(cmd!(SUDO, EOA, SH, ARG_C, ASML_LUNARVIM))
    }

    pub fn azjx_rezosur() -> Option<Expression> {
        None
    }

    pub fn eqstalx_bootloader() -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, AHG_BOOTLOADER_INSTALL))
    }
 
    pub fn eqstalx_bootloader_builder() -> Option<Expression> {
        None
    }
    

    pub fn eqstalx_fs() -> Option<Expression> { 
        Some(cmd!(SUDO, EOA, SH, ARG_C, ASML_EQSTALX_FS))
    }

    pub fn eqstalx_packages(packages: &str) -> Option<Expression> {
        let mut args_sh = String::new();
        match Path::new(&format!("{DIR_HG_ROOT}/{LOC_DB_LOCK_PACMAN}")).exists() {
            true => {
                args_sh = format!("{AHG_RM} {LOC_DB_LOCK_PACMAN};");
            },
            _ => {},
        }
        args_sh = format!("{args_sh} {AHG_PACMAN_INSTALL} {packages}");
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh))
    }
    
    pub fn bridge_arch_gap() -> Option<Expression> {
        let mut args_sh = String::new(); 
        if !Path::new(LOC_HG_QEMU_USER_STATIC).exists() {
            args_sh.push_str(AHG_INSTALL_QEMU_STATIC);
            if !Path::new(LOC_DEFAULT_BINFMT_ARCH).exists() {
                args_sh.push_str(AHG_REGISTER_QEMU_STATIC);
            }
        }
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh))
    }

    pub fn clean_up_install(arch: &str) -> Option<Expression> {
        let mut args_sh = String::new(); 
        for dir in CLEANUP_DIRS {
            args_sh = format!("{dir};"); 
        }
        args_sh = format!("{args_sh} {RM} {ARG_R} {ARG_F} {DIR_MNT}/Manjaro-ARM-{arch}-latest.tar.gz*");
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh))
    }

    pub fn dd_first_mbs(path_drive: &Path) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{DD} {IF_DEV_ZERO} of={} {ASC_DEL_LPT}", path_drive.display())))
    }

    pub fn extract(loc_file: &str, args: &str) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_TAR} {loc_file} {}", args)))
    }

    pub fn extract_rootfs(loc_file: &str, path_dir: PathBuf) -> Option<Expression> {
        match Path::new(LOC_HG_MAHRK_IMAZJ_KOQSTRUE).exists() {
            true => Self::extract(loc_file, format!("{ARG_C} {}", path_dir.display()).as_str()),
            false => Self::extract(loc_file, format!("{ARG_C} {}{TOUCH_MARKER_KOQSTRUE}", path_dir.display()).as_str()),
        }
    }

    pub fn update_packages() -> Option<Expression> {
        match Path::new(LOC_HG_MAHRK_IMAZJ_KOQSTRUE).exists() {
            true => Some(cmd!(SUDO, EOA, SH, ARG_C, AHG_PACMAN_UPDATE)),
            false => Some(cmd!(SUDO, EOA, SH, ARG_C, concatcp!(AHG_PACMAN_UPDATE, RM_MARKER_KOQSTRUE, TOUCH_MARKER_DATIZJE))), 
        }
    }

    pub fn make_dirs(dirs: &[&str]) -> Option<Expression> {
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
                    let args_sh = dirs.iter().with_position().map(|e| {
                    match e {
                        (Position::First, dir) | (Position::Middle, dir) => format!("{ASC_MKDIR_P} {}; ", dir.display()),
                        (_, dir) => format!("{ASC_MKDIR_P} {}", dir.display()),
                    }
                }).collect::<String>();
                Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh))
            },
        }
    }

    pub fn make_label(drive: &Path) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_PARTED_S} {} {ASC_MKLABEL_GPT}", drive.display())))
    }

    pub fn make_boot_partition(drive: &Path, partition_type: &str) -> Option<Expression> { 
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_BOOT_SPACE}", drive.display(), partition_type)))
    }

    pub fn make_root_partition(drive: &Path, partition_type: &str) -> Option<Expression> { 
        match drive.file_name().unwrap().to_str() {
            Some(_) => Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_END_SECTOR} {C_PERCENT}", drive.display(), partition_type))),
            _ => panic!("Cannot unwrap device name")
        }
    }

    pub fn make_subvol(drive: &Path) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_BTRFS_SUCR} {}", drive.display())))
    }

    pub fn mkfs_btrfs(drive: &Path, partition: u32) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{MKFS_BTRFS} {ARG_M} {SINGLE} {ARGS_L} {LABEL_ROOT_AND_HOME} {ARG_F} {}{partition}", drive.display())))
    }

    pub fn mkfs_vfat(drive: &Path, partition: u32) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{MKFS_VFAT} {ARG_N} {LABEL_BOOT} {}{partition}", drive.display())))
    }

    pub fn mount(drive: &Path, dir: &str) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{MOUNT} {} {dir}", drive.display())))
    }

    pub fn mount_mainvol(partition: &Path, dir: &str) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{ASC_MOUNT_O} {MAIN_VOL_COMPRESS} {} {}", partition.display(), dir)))
    }

    pub fn mount_subvols(partition: &Path, subvols: &[(&str, &str)]) -> Option<Expression> {
        let mut args_sh = String::new();
        
        let mut subvols = subvols.iter().with_position().peekable();
        while let Some(e) = subvols.next() {
            match e {
                (Position::First, subvol) | (Position::Middle, subvol) => 
                    args_sh.push_str(format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}; {ASC_MKDIR_P} {}; ", 
                        subvol.0, partition.display(), subvol.1, subvols.peek().unwrap().1.1).as_str()),
                (_, subvol) => args_sh.push_str(format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}", subvol.0, partition.display(), subvol.1).as_str()),
            }
        }
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh))
    }

    pub fn move_boot() -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, ASC_MV_BOOT))
    }

    pub fn partprobe(drive: &Path) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, format!("{PARTPROBE} {}", drive.display())))
    }

    pub fn remove_partitions_drive(drive: &Path) -> Option<Expression> {
        let dis_drive = drive.display();

        let list_pts = CommandOutput::partition_numbers(drive);

        match list_pts.len() {
            0 => None,
             _ => {
                let sh_remove: String = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}; "),
                        (_, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}"),
                    }
                }).collect();
                Some(cmd!(SUDO, EOA, SH, ARG_C, sh_remove)) 
            }
        }
    }

    pub fn set_settings_system(region_timezone: &str, zone_timezone: &str, locale: &str, keymap: &str, name_host: &str) -> Option<Expression> {
        let args_sh = format!("{ARTIX_CHROOT} {DIR_HG_ROOT} {LN} {ARG_S} {ARG_F} /usr/share/zoneinfo/timezone/{region_timezone}/{zone_timezone} /etc/localtime; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {SED} {ARG_I} s/\"#{locale}\"/\"{locale}\"/g {LOC_LOCALE_GEN}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {ECHO} \"LOCALE={locale}\" | {TEE} {ARG_A} {LOC_LOCALE_CONF}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {LOCALE_GEN} \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {ECHO} \"KEYMAP={keymap}\nFONT={DEFAULT_CONSOLEFONT}\" | {TEE} {ARG_A} {LOC_VCONSOLE_CONF}; \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {LOCALE_GEN} \
        {ARTIX_CHROOT} {DIR_HG_ROOT} {name_host} | {TEE} {ARG_A} {LOC_HOSTNAME} \
        ");
        
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh)) 
    }

    pub fn set_users(user: &str, name_full: &str, password_user: &str, password_root: &str, key_pub_user: &str) -> Option<Expression> {
        let args_sh = format!("{ARTIX_CHROOT} {DIR_HG_ROOT} {GROUPMOD} {ARG_N} {user} {DEFAULT_USERGROUP_USER}; \
            {ARTIX_CHROOT} {DIR_HG_ROOT} {USERMOD} {ARG_L} {user} {DEFAULT_USERNAME} \
            {ARG_A} {ARGS_G} {DEFAULT_USERGROUPS} {ARG_P} {password_user} {ARG_S} {DEFAULT_SHELL} \
            {ARG_M} {ARG_D} /home/{user} {ARG_C} \"{name_full}\"; \
            {ARTIX_CHROOT} {DIR_HG_ROOT} {INSTALL} {ARL_DIR} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD700} /home/{user}/.ssh; \
            {ARTIX_CHROOT} {DIR_HG_ROOT} {INSTALL} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD600} /home/{user}/.ssh/authorized_keys; \
            {ECHO} {key_pub_user} | {TEE} {ARG_A} {DIR_HG_ROOT}/home/{user}/.ssh/authorized_keys; \
            {ARTIX_CHROOT} {DIR_HG_ROOT} {INSTALL} {ARL_OWNER}{user} {ARL_GROUP}{user} {ARG_MOD644} {LOC_PROFILE} /home/{user}/.profile; \
            {ARTIX_CHROOT} {DIR_HG_ROOT} {USERMOD} {ARG_P} {password_root} {ROOT} \
        ");
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh)) 
    }

    pub fn _show_elapsed_time() -> Option<Expression> {
        None
    }
       
    pub fn umount_volume(path: &Path) -> Option<Expression> { 
        match CommandOutput::is_mounted(path) {
            true => Some(cmd!(SUDO, UMOUNT, path)),
            false => None,
        }
    }

    pub fn umount_drive(drive: &Path) -> Option<Expression> { 
        let list_mvs = CommandOutput::mounted_volumes_device(drive);

        debug!("mnt_vols: {:?}", list_mvs);

        match list_mvs.len() {
            0 => None,
             _ => {
                let sh_remove: String = list_mvs.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{UMOUNT} {}; ", partition.display()),
                        (_, partition) => format!("{UMOUNT} {}", partition.display()),
                    }
                }).collect();
                Some(cmd!(SUDO, EOA, SH, ARG_C, sh_remove)) 
            }
        }
    }

    pub fn umount_dirs(dirs: &[&str]) -> Option<Expression> {
       
        let args_sh: String = dirs.iter().with_position().map(|e| {
            match e {
                (Position::First, dir) | (Position::Middle, dir) => format!("{UMOUNT} {dir}; "),
                (_, dir) => format!("{UMOUNT} {dir}"),
            }
        }).collect();

        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh)) 
    }

    pub fn wget(dir_end: &str, url_download: &str) -> Option<Expression> {
        match Path::new(&format!("{dir_end}/{FILE_XZ_ARMTIX}")).exists() {
            false => Some(cmd!(SUDO, WGET, ARG_Q, url_download).dir(dir_end)),
            true => None,
        }
    }

    pub fn zjenx_fstab() -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, AHG_FSTABGEN).pipe(cmd!(SUDO, TEE, LOC_HG_FSTAB))) 
    }
}
