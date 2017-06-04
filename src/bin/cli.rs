extern crate the_quizz;
#[macro_use] extern crate diesel;

#[macro_use] extern crate log;
extern crate slog_envlogger;

extern crate rustbox;
extern crate rustc_serialize;

use rustbox::{Color, RustBox};
use rustbox::Key;

use the_quizz::*;

trait ValidateBox {
    fn new() -> ValidateStruct;
    fn print(&mut self, rustbox: &RustBox, key: &Key);
    fn display_result(&mut self, rustbox: &RustBox);
}

impl ValidateBox for ValidateStruct {
    fn new() -> ValidateStruct {
        let mut data_base = QuizzDataBaseStruct::new();
        let question = data_base.get_question();
        println!("{:?}", question);

        ValidateStruct {
            selected_value: false,
            x: 15, y: 5,
            entitled: question.entitled,
            response: question.response,
            content: textual_content(None)
        }
    }

    fn print(&mut self, rustbox: &RustBox, key: &Key) {
        let yes_key_press = self.content.yes_key.chars().next().unwrap();
        let no_key_press = self.content.no_key.chars().next().unwrap();

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
                          &format!("[ {} ]", self.content.no));
            rustbox.print(self.x + self.content.no.len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", self.content.yes));
        } else {
            rustbox.print(self.x,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", self.content.no));
            rustbox.print(self.x + self.content.no.len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[ {} ]", self.content.yes));
        }
    }

    fn display_result(&mut self, rustbox: &RustBox) {
        if self.selected_value == self.response {
            rustbox.print(self.x + self.content.no.len() + 6 + self.content.yes.len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Green,
                  Color::Black,
                  &*self.content.checked);
        }
        else {
            rustbox.print(self.x + self.content.no.len() + 6 + self.content.yes.len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Red,
                  Color::Black,
                  &*self.content.unchecked);
        }
    }
}


fn main() {
    slog_envlogger::init().unwrap();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let mut validate_box = ValidateStruct::new();

    rustbox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &*validate_box.content.title);
    rustbox.print(80,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Cyan,
                  &*validate_box.content.quit);
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
