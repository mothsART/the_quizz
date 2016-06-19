extern crate gettext;
extern crate postgres;

use std::fs::File;

pub use self::gettext::Catalog;

pub use self::postgres::{Connection, SslMode};

pub struct Question {
    pub id: i32,
    pub entitled: String,
    pub response: bool
}

pub struct ValidateStruct {
    pub x: usize,
    pub y: usize,
    pub selected_value: bool,
    pub entitled: String,
    pub response: bool
}

pub trait QuizzDataBase {
    fn new() -> QuizzDataBaseStruct;
    fn get_question(&mut self) -> Question;
    fn get_response(&mut self, question: Question) -> bool;
}

#[derive(Debug)]
pub struct QuizzDataBaseStruct {
    pub connection: postgres::Connection
}

impl QuizzDataBase for QuizzDataBaseStruct {
    fn new() -> QuizzDataBaseStruct {
        let connection = Connection::connect("postgresql://postgres@localhost", SslMode::None).unwrap();
        QuizzDataBaseStruct {
            connection: connection
        }
    }

    /// get a random question
    fn get_question(&mut self) -> Question {
        let mut question = Question { id: 0, entitled: String::from(""), response: false };
        for row in &self.connection.query("SELECT id, entitled, response FROM questions ORDER BY random() LIMIT 1", &[]).unwrap() {
            question = Question {
                id: row.get(0),
                entitled: row.get(1),
                response: row.get(2)
            };
        }
        question
    }

    /// give response of a question
    fn get_response(&mut self, question: Question) -> bool {
        let mut response = false;
        for row in &self.connection.query("SELECT response FROM questions WHERE id=$1 LIMIT 1", &[&question.id]).unwrap() {
            response = row.get(0)
        }
        response
    }
}

#[derive(Debug)]
pub enum TextualContent {
    Title,
    Quit,
    Yes,
    No,
    YesKey,
    NoKey,
    Checked,
    UnChecked
}

impl TextualContent {
   pub fn str(&self) -> String {
        let f = File::open("locale/fr/LC_MESSAGES/messages.mo").expect("could not open the catalog");
        let catalog = Catalog::parse(f).expect("could not parse the catalog");
        match *self {
            TextualContent::Title => catalog.gettext("The QuiiiZz !").to_string(),
            TextualContent::Quit => catalog.gettext("Press 'q' to quit.").to_string(),
            TextualContent::Yes => catalog.gettext("Yes").to_string(),
            TextualContent::No => catalog.gettext("No").to_string(),
            TextualContent::YesKey => catalog.gettext("y").to_string(),
            TextualContent::NoKey => catalog.gettext("n").to_string(),
            TextualContent::Checked => "✔".to_string(),
            TextualContent::UnChecked => "✘".to_string()
        }
   }
}