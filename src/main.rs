extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use lib::{QuizzDataBase, QuizzDataBaseStruct, ValidateStruct, TextualContent};
mod lib;

trait ValidateBox {
    fn new() -> ValidateStruct;
    fn print(&mut self, rustbox: &RustBox, key: &Key);
    fn display_result(&mut self, rustbox: &RustBox);
}

impl ValidateBox for ValidateStruct {
    fn new() -> ValidateStruct {
        let mut data_base = QuizzDataBaseStruct::new();
        let question = data_base.get_question();

        ValidateStruct {
            selected_value: false,
            x: 15, y: 5,
            entitled: question.entitled, response: question.response
        }
    }

    fn print(&mut self, rustbox: &RustBox, key: &Key) {
        let yes_key_press = TextualContent::YesKey.str().chars().next().unwrap();
        let no_key_press = TextualContent::NoKey.str().chars().next().unwrap();

        if *key == Key::Left && self.selected_value == true {
            self.selected_value = false;
        } else if *key == Key::Right && self.selected_value == false {
            self.selected_value = true;
        } else if *key == Key::Enter {
            self.display_result(rustbox);
        } else if *key == Key::Char(yes_key_press) {
            self.selected_value = true;
            self.display_result(rustbox);
        } else if *key == Key::Char(no_key_press) {
            self.selected_value = false;
            self.display_result(rustbox);
        }

        rustbox.print(self.x,
                      4,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &self.entitled);

        if self.selected_value == true {
            rustbox.print(self.x,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[ {} ]", TextualContent::No.str()));
            rustbox.print(self.x + TextualContent::No.str().len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", TextualContent::Yes.str()));
        } else {
            rustbox.print(self.x,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", TextualContent::No.str()));
            rustbox.print(self.x + TextualContent::No.str().len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[ {} ]", TextualContent::Yes.str()));
        }
    }

    fn display_result(&mut self, rustbox: &RustBox) {
        if self.selected_value == self.response {
            rustbox.print(self.x + TextualContent::No.str().len() + 6 + TextualContent::Yes.str().len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Green,
                  Color::Black,
                  &*TextualContent::Checked.str());
        }
        else {
            rustbox.print(self.x + TextualContent::No.str().len() + 6 + TextualContent::Yes.str().len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Red,
                  Color::Black,
                  &*TextualContent::UnChecked.str());
        }
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &*TextualContent::Title.str());
    rustbox.print(80,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Cyan,
                  &*TextualContent::Quit.str());

    let mut validate_box = ValidateStruct::new();
    validate_box.print(&rustbox, &Key::Unknown(0));

    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {
                        validate_box.print(&rustbox, &key);
                    }
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
