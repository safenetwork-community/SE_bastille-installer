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
// const GRUB_INSTALL: &str = "grub-install";
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
const PARTED: &str = "parted";
const PARTPROBE: &str = "partprobe";
const SED: &str = "sed";
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
const ARG_X: &str = "-x";
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
const RM_MARKER_KOQSTRUE: &str = formatcp!("{RM} {LOC_HG_MAHRK_IMAZJ_KOQSTRUE}");
// const RM_MARKER_DATIZJE: &str = formatcp!(SC_SPACE, RM, LOC_HG_MAHRK_IMAZJ_DATIZJE);

// marker touch commands
const TOUCH_MARKER_KOQSTRUE: &str = formatcp!("{TOUCH} {LOC_HG_MAHRK_IMAZJ_KOQSTRUE}");
const TOUCH_MARKER_DATIZJE: &str = formatcp!("{TOUCH} {LOC_HG_MAHRK_IMAZJ_DATIZJE}");

// cleanup dirs
pub const CLEANUP_CMDS: [&str; 11] = [
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
    formatcp!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/{LOC_HG_MAHRK_IMAZJ_FINI}")
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
// const ASC_MV_BOOT: &str = formatcp!("{MV} {DIR_HG_ROOT}/{BOOT}/* {DIR_HG_BOOT}");
// const ASC_PACMAN_KEY_INIT: &str = formatcp!("{PACMAN_KEY} {ARL_INIT}");
// const ASC_PACMAN_KEY_POPULATE: &str = formatcp!("{PACMAN_KEY} {ARL_POPULATE} {ARTIX_ARM}");
const ASC_PARTED_OPT: &str = formatcp!("{PARTED} {ARG_A} {OPTIMAL} {ARG_S}");
const ASC_PARTED_S: &str = formatcp!("{PARTED} {ARG_S}");
const ASC_TAR_EXTRACT: &str = formatcp!("{TAR} {ARG_X}f");

// sh hg arguments
// const AHG_BOOTLOADER_INSTALL_GRUB_UEFI: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {GRUB_INSTALL} --target=arm64-efi \
// --efi-directory=efi --removable --bootloader-id=grub");
// const AHG_BOOTLOADER_INSTALL_GRUB_BIOS: &str = "{ARTIX_CHROOT} {DIR_HG_ROOT} {GRUB_INSTALL} --recheck)";
// const AHG_BOOTLOADER_CONFIG_GRUB: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} grub-mkconfig {ARG_O} /boot/grub/grub.cfg");
const AHG_FSTABGEN: &str = formatcp!("{FSTABGEN} {ARGS_U} {DIR_HG_ROOT}");
const AHG_INSTALL_QEMU_STATIC: &str = formatcp!("{INSTALL} {ARG_MOD755} {ARGS_C} {LOC_QEMU_USER_STATIC} {LOC_HG_QEMU_USER_STATIC}");
// const AHG_MOUNT: &str = formatcp!("{ARTIX_CHROOT} {DIR_HG_ROOT} {MOUNT}");
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
const ASML_EQSTALX_FS: &str = formatcp!("{AHG_RM_STS}; {AHG_PACMAN_INSTALL} {DEFAULT_PACKAGE_FS}");

#[derive(Clone, Copy)]
pub struct CommandAction {}
    
impl CommandAction {

    pub fn azjx_editor() -> Option<Expression> {
        Self::get_action_str(ASML_LUNARVIM)
    }

    pub fn azjx_rezosur() -> Option<Expression> {
        None
    }

    // pub fn eqstalx_bootloader() -> Option<Expression> {
        /*
        let args_sh = format!("{AHG_MOUNT} /dev/sda2 /boot; {}", match firmware {
            BL_COREBOOT => format!(""), // TODO: implement open source bootloader
            BL_U_BOOT => format!(""), // TODO: implement open source bootloader
            BL_UEFI => format!("{AHG_PACMAN_INSTALL} {DEFAULT_PACKAGE_BOOTLOADER_GRUB}; \
                {AHG_BOOTLOADER_INSTALL_GRUB_UEFI}; {AHG_BOOTLOADER_CONFIG_GRUB}"),
            _ => format!("{AHG_PACMAN_INSTALL} {DEFAULT_PACKAGE_BOOTLOADER_GRUB}; \
                {AHG_BOOTLOADER_INSTALL_GRUB_BIOS}; {AHG_BOOTLOADER_CONFIG_GRUB}"),
        });*/
        // Self::get_action(args_sh)
   // }
 
    pub fn eqstalx_fs() -> Option<Expression> { 
        Self::get_action_str(ASML_EQSTALX_FS)
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
        Self::get_action(args_sh)
    }
    
    pub fn bridge_arch_gap() -> Option<Expression> {
        let mut args_sh = String::new(); 
        if !Path::new(LOC_HG_QEMU_USER_STATIC).exists() {
            args_sh.push_str(AHG_INSTALL_QEMU_STATIC);
            if !Path::new(LOC_DEFAULT_BINFMT_ARCH).exists() {
                args_sh.push_str(AHG_REGISTER_QEMU_STATIC);
            }
        }
        Self::get_action(args_sh)
    }

    pub fn clean_up_install(init: &str) -> Option<Expression> {
        let mut args_sh = CLEANUP_CMDS.iter().fold(String::new(), |acc, command| acc + command + SC_SPACE);
        args_sh.push_str(format!("{RM} {ARG_R} {ARG_F} {DIR_MNT}/armtix-{init}-2024*").as_str());
        Self::get_action(args_sh)
    }

    pub fn dd_first_mbs(path_drive: &Path) -> Option<Expression> {
        Self::get_action(format!("{DD} {IF_DEV_ZERO} of={} {ASC_DEL_LPT}", path_drive.display()))
    }

    pub fn extract_rootfs(loc_file: &str, path_dir: PathBuf) -> Option<Expression> {
        let sh_args = match Path::new(LOC_HG_MAHRK_IMAZJ_KOQSTRUE).exists() {
            true => format!("{ARGS_C} {}", path_dir.display()),
            false => format!("{ARGS_C} {}; {TOUCH_MARKER_KOQSTRUE}", path_dir.display()),
        };
        Self::get_action(format!("{} {}", Self::get_tar_extract(loc_file), sh_args))
    }

    pub fn update_packages() -> Option<Expression> {
        Self::get_action(format!("{AHG_PACMAN_UPDATE}{}", match Path::new(LOC_HG_MAHRK_IMAZJ_KOQSTRUE).exists() {
            false => concatcp!(SC_SPACE, RM_MARKER_KOQSTRUE, TOUCH_MARKER_DATIZJE), 
            _ => ""
        }))
    }

    pub fn make_label(drive: &Path) -> Option<Expression> {
        Self::get_action(format!("{ASC_PARTED_S} {} {ASC_MKLABEL_GPT}", drive.display()))
    }

    pub fn make_boot_partition(drive: &Path, partition_type: &str) -> Option<Expression> { 
        Self::get_action(format!("{ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_BOOT_SPACE}", drive.display(), partition_type))
    }

    pub fn make_root_partition(drive: &Path, partition_type: &str) -> Option<Expression> { 
        match drive.file_name().unwrap().to_str() {
            Some(_) => Self::get_action(format!("{ASC_PARTED_OPT} {} {ASC_MKPART_PRIMARY} {} {ASC_END_SECTOR} {C_PERCENT}", drive.display(), partition_type)),
            _ => panic!("Cannot unwrap device name")
        }
    }

    pub fn mkfs_btrfs(drive: &Path, partition: u32) -> Option<Expression> {
        Self::get_action(format!("{MKFS_BTRFS} {ARG_M} {SINGLE} {ARGS_L} {LABEL_ROOT_AND_HOME} {ARG_F} {}{partition}", drive.display()))
    }

    pub fn mkfs_vfat(drive: &Path, partition: u32) -> Option<Expression> {
        Self::get_action(format!("{MKFS_VFAT} {ARG_N} {LABEL_BOOT} {}{partition}", drive.display()))
    }

    pub fn mount_drive(vols: Vec<(&str, &Path, &Path)>, subvols: &[(&str, &Path)]) -> Option<Expression> {
        let (one, two, three): (Vec<_>, Vec<_>, Vec<_>) = vols.into_iter().multiunzip();
        let mut args_sh = String::new();
        args_sh.push_str(&Self::get_make_dirs(vols.into_iter().multiunzip().1));
        args_sh.push_str(&format!("; {}", &Self::get_mount_mainvol(vols[0].1, vols[0].2)));
        subvols.iter().for_each(|(name_dir, path)| 
            args_sh.push_str(&format!("; {ASC_BTRFS_SUCR} {}", 
            path.display())));
        Self::get_action(args_sh)
    }

    pub fn mount_drive_part_two(ppt_root: &Path, sub_vols: &[(&Path, &str)], mnt_boot: (&Path, &Path)) -> Option<Expression> {
        let mut args_sh = String::new();

        let mut subvols = sub_vols.iter().with_position().peekable();
        while let Some(e) = subvols.next() {
            match e {
                (Position::First, subvol) | (Position::Middle, subvol) => 
                args_sh.push_str(&format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}; {ASC_MKDIR_P} {}; ", 
                    subvol.1, ppt_root.display(), subvol.0.display(), subvols.peek().unwrap().1.0.display())),
                (_, subvol) => args_sh.push_str(&format!("{ASC_MOUNT_O} {SUB_VOL_COMPRESS}{} {} {}", 
                    subvol.1, ppt_root.display(), subvol.0.display())),
            }
        }

        args_sh.push_str(&format!("; {}", Self::get_mount(mnt_boot.0, mnt_boot.1)));
        Self::get_action(args_sh)   
    }

    pub fn partprobe(drive: &Path) -> Option<Expression> {
        Self::get_action(format!("{PARTPROBE} {}", drive.display()))
    }

    pub fn remove_partitions_drive(drive: &Path) -> Option<Expression> {
        let dis_drive = drive.display();

        let list_pts = CommandOutput::partition_numbers(drive);

        match list_pts.len() {
            0 => None,
            _ => {
                let args_sh = list_pts.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}; "),
                        (_, partition) => format!("{ASC_PARTED_S} {dis_drive} {RM} {partition}"),
                    }
                }).collect();
                Self::get_action(args_sh)
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
        Self::get_action(args_sh)
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
        Self::get_action(args_sh)
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
                let args_sh: String = list_mvs.iter().with_position().map(|e| {
                    match e {
                        (Position::First, partition) | (Position::Middle, partition) => format!("{UMOUNT} {}; ", partition.display()),
                        (_, partition) => format!("{UMOUNT} {}", partition.display()),
                    }
                }).collect();
                Self::get_action(args_sh)
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

        Self::get_action(args_sh)
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

    fn get_action_str(args_sh: &str) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh)) 
    }

    fn get_action(args_sh: String) -> Option<Expression> {
        Some(cmd!(SUDO, EOA, SH, ARG_C, args_sh)) 
    }
    
    fn get_make_dirs(paths: Vec<&Path>) -> String {
        paths.iter().with_position().fold(String::new(), |acc, (position, path)| match position {
            Position::First | Position::Middle => format!("{acc} {ASC_MKDIR_P} {}; ", path.display()),
            _ => format!("{acc} {ASC_MKDIR_P} {}", path.display()),
        })
    }

    fn get_mount(drive: &Path, mountpoint: &Path) -> String {
        format!("{MOUNT} {} {}", drive.display(), mountpoint.display())
    }

    fn get_mount_mainvol(partition: &Path, mountpoint: &Path) -> String {
        format!("{ASC_MOUNT_O} {MAIN_VOL_COMPRESS} {} {}", partition.display(), mountpoint.display())
    }

    fn get_tar_extract(loc_file: &str) -> String {
        format!("{ASC_TAR_EXTRACT} {loc_file}")
    }
}
