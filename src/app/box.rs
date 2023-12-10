// dboxes, mboxes and exits
#[derive(Clone)]
pub enum Page {
    Drive, EmptyHostname, EmptyFullname,
    EmptyMenu, EmptyPasswordRoot, 
    EmptyPasswordUser, EmptyUsername,
    Escape, Finish, Fullname, Hostname,
    InvalidHostname, InvalidUsername,
    KeymapGuest, KeymapHost, 
    KeyvarGuest, KeyvarHost, NoBoxFound, 
    NoMatchPasswordRoot, NoMatchPasswordUser,
    MenuConfig, MenuMain,
    PasswordUserSgn, PasswordUserRpt,
    PasswordRootSgn, PasswordRootRpt,
    QuestionConfig, Quit,
    TimezoneRegion, TimezoneZone,
    UnknownError, Usergroups, Username
}
