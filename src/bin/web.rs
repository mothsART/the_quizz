#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate the_quizz;
extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;
use rocket::response::NamedFile;
use rocket_contrib::{JSON, MsgPack};
use rocket::response::content;
use rocket::config::{Config, Environment};
use std::path::{Path, PathBuf};
use the_quizz::*;
use the_quizz::models::{Question};

struct QuizzRequest {
    indice: i32,
    choice: String
}

#[derive(Serialize)]
struct QuizzEntitledResponse {
    indice: i32,
    entitled: String
}

#[derive(Serialize)]
struct QuizzReplyResponse {
    response: bool
}

fn get_env() -> String {
    let environment = match ::std::env::var("ROCKET_ENV") {
        Ok(name) => {
            name
        },
        Err(_) => {
            "development".to_string()
        }
    };
    environment
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/output/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    match &*get_env() {
        "development" => {
            NamedFile::open(Path::new("static/output").join(file)).ok()
        },
        _ => {
            NamedFile::open(Path::new("static/output").join(file)).ok()
        }
    }
}

#[get("/<file..>")]
fn fonts(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/app/fonts").join(file)).ok()
}

#[get("/<file..>")]
fn img(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/app/img").join(file)).ok()
}

#[post("/", format = "application/msgpack")]
fn question() ->  MsgPack<QuizzEntitledResponse> {
    let mut data_base = QuizzDataBaseStruct::new();
    let question = data_base.get_question();
    MsgPack(QuizzEntitledResponse {
        indice: question.id,
        entitled: question.entitled
    })
}

#[post("/", data = "<data>", format = "application/msgpack")]
fn reply(data:  MsgPack<i32>) -> MsgPack<bool> {
    let mut data_base = QuizzDataBaseStruct::new();
    let question = Question {
        id: data.0, entitled: String::from(""),
        response: false, explanation: None,
        source: None
    };
    let mut reply = data_base.get_response(question);
    //println!("{:?}", MsgPack(reply));
    /*let content = textual_content(None);
    let yes = content.yes;
    let mut response = false;
    match &*quizz_request.choice {
        msg if msg == yes => {
            response = true;
        },
        _ => {}
    }
    let mut good_response = false;
    if reply == response {
        good_response = true;
    }*/
    MsgPack(
        reply
    )
}

/*
fn i18n_strings(_request: &mut Request) -> IronResult<Response> {
    let content = textual_content(None);
    Ok(Response::with((status::Ok, json::encode(&content).unwrap())))
}
*/

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
    .mount("/static/output/", routes![files])
    .mount("/static/fonts/", routes![fonts])
    .mount("/static/img/", routes![img])
    .mount("/question", routes![question])
    .mount("/reply", routes![reply])
}

fn main() {
    rocket().launch();
}
