#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate log;

extern crate serde;
extern crate dotenv;
extern crate gettext;
extern crate walkdir;

pub use self::walkdir::WalkDir;
pub use self::gettext::{Catalog, Error};

pub use rocket_contrib::MsgPack;
pub use serde::Serialize;

pub mod schema;
pub mod models;

use std::fs::File;
use diesel::prelude::*;
use diesel::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Question};
use self::schema::questions::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug)]
pub struct TextualContent {
    pub title: String,
    pub quit: String,
    pub  yes: String,
    pub no: String,
    pub yes_key: String,
    pub no_key: String,
    pub checked: String,
    pub unchecked: String
}

pub struct ValidateStruct {
    pub x: usize,
    pub y: usize,
    pub selected_value: bool,
    pub entitled: String,
    pub response: bool,
    pub content: TextualContent
}

pub trait QuizzDataBase {
    fn new() -> QuizzDataBaseStruct;
    fn get_question(&mut self) -> Question;
    fn get_response(&mut self, question: Question) -> bool;
}

pub struct QuizzDataBaseStruct {
    pub connection: PgConnection
}

impl QuizzDataBase for QuizzDataBaseStruct {
    fn new() -> QuizzDataBaseStruct {
        QuizzDataBaseStruct {
            connection:  establish_connection()
        }
    }

    /// get a random question
    fn get_question(&mut self) -> Question {
        no_arg_sql_function!(random, types::VarChar);
        let query_fragment = questions.limit(1).order(random);
        info!("{:?}", &debug_sql!(query_fragment));
        let b = query_fragment.first(&self.connection).unwrap();
        b
    }

    /// give response of a question
    fn get_response(&mut self, question: Question) -> bool {
        let r = questions.filter(id.eq(question.id)).limit(1).load::<Question>(&self.connection).unwrap();
        info!("{:?}", r[0].response);
        r[0].response
    }
}

fn trans(catalog: &Option<Result<Catalog, Error>>, content: &'static str) -> String {
    return match *catalog {
        Some(ref catalog) => {
            match *catalog {
                Ok(ref c) => {
                    c.gettext(&*content).to_string()
                }
                _ => {
                    String::from(content)
                }
            }
        }
        _ => {
            String::from(content)
        }
    };
}

pub fn textual_content(force_lang: Option<String>) -> TextualContent {
    let lang = match force_lang {
        Some(l) => {
            l
        }
        None => {
            env::var("LANGUAGE").unwrap()
        }
    };
    let path = &*format!(
        "locale/{}/LC_MESSAGES/messages.mo",
        lang
    );
    let catalog = match File::open(&path) {
        Ok(f) => {
            Some(Catalog::parse(f))
        }
        _ => {
            None
        }
    };

    TextualContent {
        title: trans(&catalog, "The QuiiiZz !"),
        quit:  trans(&catalog, "Press 'q' to quit."),
        yes: trans(&catalog, "Yes"),
        no: trans(&catalog, "No"),
        yes_key: trans(&catalog, "y"),
        no_key: trans(&catalog, "n"),
        checked: trans(&catalog, "✔"),
        unchecked: trans(&catalog, "✘")
    }
}

pub fn langs() -> Vec<String> {
    let mut langs: Vec<String> = Vec::new();
    for entry in WalkDir::new("locale") {
        match entry {
            Ok(e) => {
                println!("{}", e.path().display());
            }
            _ => {
                println!("rrrr");
            }
        };
    };
    return langs;
}
