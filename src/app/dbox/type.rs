use dialog::{backends::Dialog, Choice, DialogBox, Question};

use crate::app::constants::{TITLE, TITRFOQ};

// Dimensions question box
const DEFAULT_WIDTH_BOX_QUESTION: u32 = 90;
const DEFAULT_HEIGHT_BOX_QUESTION: u32 = 20;

// Dialog box unwrap failure text 
const EXP_DBOX: &str = "Could not display dialog box.";

// Types of boxes
/*pub enum BoxTypeMenu {
    Default, Main
}*/

pub trait Default {
    fn get_default_box() -> Dialog; 
}

pub trait QuestionBoxChoice {
    fn choice(text: String, dbox: Option<Dialog>) -> Choice; 
}

pub trait InputBoxChoice {
    fn choice(text: String, default: &str, dbox: Option<Dialog>) -> (Choice, Option<String>);
}

pub trait PasswordBoxChoice {
    fn choice(text: String, dbox: Option<Dialog>) -> (Choice, Option<String>); 
}

/*
pub trait MenuBoxChoice {
    fn choice(box_type: BoxTypeMenu, text: String, list: Vec<[String; 2]>) -> (Choice, Option<String>); 
}
*/
pub struct QuestionBox {}

impl Default for QuestionBox {
    fn get_default_box() -> Dialog {
        let mut dialog = Dialog::new();
        dialog.set_backtitle(TITRFOQ);
        dialog.set_title(TITLE);
        dialog.set_width(DEFAULT_WIDTH_BOX_QUESTION);
        dialog.set_height(DEFAULT_HEIGHT_BOX_QUESTION);
        dialog
    }
}

impl QuestionBoxChoice for QuestionBox {
    fn choice(text: String, dbox: Option<Dialog>) -> Choice {
        match dbox {
            Some(qbox) => Question::new(text).show_with(&qbox).expect(EXP_DBOX),
            None => Question::new(text).show_with(Self::get_default_box()).expect(EXP_DBOX),
        }
    }
}
