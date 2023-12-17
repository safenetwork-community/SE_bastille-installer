// dboxes, mboxes and exits
#[derive(Clone)]
pub enum Page {
    EmptyHostname, EmptyFullname,
    EmptyMenu, EmptyPasswordRoot, 
    EmptyPasswordUser, EmptyUsername,
    Escape, Finish, 
    InputFullname, InputHostname,
    InputUsergroups, InputUsername,
    InvalidHostname, InvalidUsername,
    MenuConfig, MenuDrive, 
    MenuKeymapGuest, MenuKeymapHost, 
    MenuKeyvarGuest, MenuKeyvarHost, 
    MenuMain, 
    MenuTimezoneRegion, MenuTimezoneZone,
    NoBoxFound, NoMatchPasswordRoot, NoMatchPasswordUser,
    PasswordUserSgn, PasswordUserRpt,
    PasswordRootSgn, PasswordRootRpt,
    QuestionConfig, Quit, UnknownError, }
