extern crate postgres;
extern crate rustbox;
extern crate gettext;

use std::fs::File;
use std::default::Default;

use postgres::{Connection, SslMode};

use rustbox::{Color, RustBox};
use rustbox::Key;

use gettext::Catalog;

struct Question {
    entitled: String,
    response: bool
}

struct ValidateStruct {
    x: usize,
    y: usize,
    catalog: gettext::Catalog,
    selected_value: bool,
    entitled: String,
    response: bool
}

trait ValidateBox<'a> {
    fn new() -> ValidateStruct;
    fn print(&mut self, rustbox: &RustBox, key: &Key);
    fn display_result(&mut self, rustbox: &RustBox);
}

impl<'a> ValidateBox<'a> for ValidateStruct {
    fn new() -> ValidateStruct {
        let f = File::open("locale/fr/LC_MESSAGES/messages.mo")
                    .expect("could not open the catalog");

        let mut question = Question { entitled: String::from(""), response: false };
        let conn = Connection::connect("postgresql://postgres@localhost", SslMode::None).unwrap();
        for row in &conn.query("SELECT entitled, response FROM questions ORDER BY random() LIMIT 1", &[]).unwrap() {
            question = Question {
                entitled: row.get(0),
                response: row.get(1),
            };
        }

        ValidateStruct {
            selected_value: false,
            catalog: Catalog::parse(f).expect("could not parse the catalog"),
            x: 15, y: 5,
            entitled: question.entitled, response: question.response
        }
    }

    fn print(&mut self, rustbox: &RustBox, key: &Key) {
        let yes_key_press = self.catalog.gettext("y").chars().next().unwrap();
        let no_key_press = self.catalog.gettext("n").chars().next().unwrap();

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
                          &format!("[ {} ]", self.catalog.gettext("No")));
            rustbox.print(self.x + self.catalog.gettext("No").len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", self.catalog.gettext("Yes")));
        } else {
            rustbox.print(self.x,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::Black,
                          Color::White,
                          &format!("[ {} ]", self.catalog.gettext("No")));
            rustbox.print(self.x + self.catalog.gettext("No").len() + 6,
                          self.y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[ {} ]", self.catalog.gettext("Yes")));
        }
    }

    fn display_result(&mut self, rustbox: &RustBox) {
        if self.selected_value == self.response {
            rustbox.print(self.x + self.catalog.gettext("No").len() + 6 + self.catalog.gettext("Yes").len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Green,
                  Color::Black,
                  "✔");
        }
        else {
            rustbox.print(self.x + self.catalog.gettext("No").len() + 6 + self.catalog.gettext("Yes").len() + 6,
                  self.y,
                  rustbox::RB_BOLD,
                  Color::Red,
                  Color::Black,
                  "✘");
        }
    }
}

fn main() {
    let f = File::open("locale/fr/LC_MESSAGES/messages.mo").expect("could not open the catalog");
    let catalog = Catalog::parse(f).expect("could not parse the catalog");

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  catalog.gettext("The QuiiiZz !"));
    rustbox.print(80,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Cyan,
                  catalog.gettext("Press 'q' to quit."));

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
