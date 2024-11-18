#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::path::Path;
    
    use const_format::formatcp;
    use duct::{cmd, Expression};
    
    // logger
    use log4rs;

    use crate::app::commands::run::TypeCommandRun;
    use crate::app::install::ListCommand;
    use crate::shared::constants::command::*;
    use crate::shared::constants::error::ErrorInstaller::{ErrorRunCommand, FailedRunCommand};
    use crate::shared::constants::install::{TXT_USERS, DIR_HG_ROOT};

    // input
    pub const KEY_PUB: &str = "id_pjehrsohmehj_folaht.pub";
    pub const LOCALE: &str = "be_FR.utf8";
    pub const NAME_DRIVE: &str = "sda";
    pub const NAME_DEVICE: &str = "test";
    pub const NAME_FULL: &str = "Fôlat Pjêrsômêj";
    pub const NAME_HOST: &str = "Rezosur-uq";
    pub const NAME_USER: &str = "folaht";
    pub const PASSWORD_ROOT: &str = "mopahsrasin";
    pub const PASSWORD_USER: &str = "mopahs";
    pub const TIMEZONE_UQKEH: &str = "Europe";
    pub const TIMEZONE_DYKEH: &str = "Amsterdam";
    pub const VAR_KEY_GUEST: &str = "yr-af";

    // path locations
    pub const LOC_HOME: &str = "/home";
    pub const TEST_LOC_HOME_USER: &str = formatcp!("{LOC_HOME}/{NAME_USER}");
    pub const TEST_LOC_SSH_USER: &str = formatcp!("{TEST_LOC_HOME_USER}/.ssh");
    pub const TEST_LOC_AKEYS_USER: &str = formatcp!("{TEST_LOC_SSH_USER}/authorized_keys");

    // path hg locations
    pub const TEST_LOC_HG_HOME_USER: &str = formatcp!("{DIR_HG_ROOT}{TEST_LOC_HOME_USER}");
    pub const TEST_LOC_HG_AKEYS: &str = formatcp!("{DIR_HG_ROOT}{TEST_LOC_AKEYS_USER}");

    // test commands and arguments
    pub const ARG_LHA: &str = "-lha";
    pub const LS: &str = "ls";
    pub const STAT: &str = "stat";
    pub const PERC_A: &str = "%a";
    
    // test commands and arguments
    pub const ERR_STAT: &str = "Unable to stat";

    #[test]
    fn test_run() {
        log4rs::init_file("./config/log4rs.yml", Default::default()).unwrap();

        let home_path = Path::new(TEST_LOC_HG_HOME_USER);
        let auth_keys = Path::new(TEST_LOC_HG_AKEYS);

        let builder_list_command = ListCommand::new(
            NAME_DEVICE, NAME_USER, NAME_FULL, 
            PASSWORD_USER, PASSWORD_ROOT,
            KEY_PUB, Path::new(&format!("/dev/{}", NAME_DRIVE)), 
            VAR_KEY_GUEST, LOCALE, 
            TIMEZONE_UQKEH, TIMEZONE_DYKEH,
            NAME_HOST);
        let dydeh_command = builder_list_command.get_dydeh_command(); 
    
        let c_start = match builder_list_command.get_markers_progress()
            .iter().find(|(file_marker, _)| file_marker.exists()) {
            Some((_, text_marker)) => {
                match dydeh_command.iter().position(|(text_progress, _)| text_progress == text_marker) {
                    Some(p) => p+1,
                    _ => 0,
                }
            }, 
            _ => 0,
        };


        dydeh_command[c_start..].iter().enumerate().for_each(|(i, (text, deh_command))| {
            debug!("text: {}", text);
            
            deh_command.iter().enumerate().for_each(|(j, command_opt)| {
                match command_opt.prepare() {
                    TypeCommandRun::Syl(command) => handle_command(command, i, j),
                    TypeCommandRun::Deh(commands) => commands.iter().for_each(|command| {
                        handle_command(command.into(), i, j);
                    }),
                    TypeCommandRun::Kuq() => {},
                }

            });

            match text.as_str() {
                TXT_USERS => {
                    match cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, LS, ARG_LHA, LOC_HOME).read() {
                        Ok(s) => debug!("{s}"),
                        Err(_) => panic!("test"),
                    }
                    assert_eq!(home_path.exists(), true);
                    assert_eq!(home_path.is_dir(), true);
                    assert_eq!(auth_keys.exists(), true);
                    assert_eq!(auth_keys.is_file(), true);
                    match cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, STAT, ARG_C, PERC_A, TEST_LOC_SSH_USER).read() {
                        Ok(s) => assert_eq!(s, "700"),
                        Err(_) => panic!("{ERR_STAT} {TEST_LOC_SSH_USER}"),
                    }
                    match cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, STAT, ARG_C, PERC_A, TEST_LOC_AKEYS_USER).read() {
                        Ok(s) => assert_eq!(s, "600"),
                        Err(_) => panic!("{ERR_STAT} {TEST_LOC_AKEYS_USER}"),
                    }
                }
                _ => {}
            }

        });

        
        // assert_eq!(cmd!(SUDO, ARTIX_CHROOT, DIR_HG_ROOT, STAT, ARG_C, PERC_A, auth_keys).read(), 644);
    }

    fn handle_command(command: Expression, i: usize, j: usize) {
        debug!("command: {:?}", command);
        match command.stdout_capture().stderr_capture().unchecked().run() {
            Ok(result_command) => {
                debug!("{}.{}, display_command: {:?}", i, j, command);
                match result_command.status.success() {
                    true => {},
                    false => {
                        error!("{}.{}, result_command: {:?}", i, j, result_command);
                        panic!("{}", ErrorRunCommand(format!("Error step: {}.{}\nProcess returned an error:\n\n{:?}\n\nOutput stderr:\n\n{}", 
                            i,j,
                            command, 
                            String::from_utf8(result_command.stderr).map_err(|non_utf8|
                            String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()).unwrap())
                        ));
                    },
                }
            },
            Err(e) => panic!("{}", FailedRunCommand(format!("Step: {}.{}\n\n{:?}\n\n{}", i, j, command, e))),
        }

    }
}


