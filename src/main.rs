use dialog::DialogBox;
use regex::RegexSet;
use std::{env, io::Error};
use std::process::Command;

fn main() -> Result<(), Error> {
    let os = "SE Bastille installer";
    let version ="0.1.0";
    let title = format!("{} {}", os, version);
    let titr = format!("Èstalxr d sistêmbêstur d Bastij {}", version);

    env::set_var("DIALOGRC", "./dialogrc_gui");

    
    loop {

        let username_widget = username_dialog(titr.clone(), title.clone()); 

        match username_widget {
            Some(name) => {

                if RegexSet::new([r"[A-Z]", r"^[0-9]"]).unwrap().is_match(&name) {
                    Command::new("clear").status().unwrap();
                    dialog::Message::new("Username contains invalid characters.").show();                    
                    continue;
                } else {
                    Command::new("clear").status().unwrap();
                    println!("Hello {}!", name);
                    break;
                }
            },
            None => { 
                Command::new("clear").status().unwrap();
                println!("==> Cancel pressed, exiting..");
                return Ok(());
            },
        };
    }

    // println!("${USER} Configuration aborted! Username contained invalid characters." {});
    println!("==> Installation finished! Terminating..");
    Ok(())
}

/*
fn msg(msg: String) -> Result<(), Error> {
    println!("{}\n==> Installation aborted! Exiting.." msg);
    Ok(())
}*/

// Keymap host dialog
fn username_dialog(titr: String, title: String) -> Option<String> {
    let mut widget = dialog::backends::Dialog::new();
    widget.set_backtitle(titr);
    widget.set_width(90);
    widget.set_height(10);

    dialog::Input::new("Enter the username you want: \
    (usernames must be all lowercase and first character may not be a number)") 
    .title(title)
    .show_with(&widget).expect("Could not display dialog box.")
}


