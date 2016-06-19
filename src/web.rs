#[macro_use]
extern crate iron;

#[macro_use]
extern crate hyper;

extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;

use std::io::Read;
use std::path::Path;

use iron::prelude::*;
use iron::status;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::headers::ContentType;
use iron::AfterMiddleware;

use std::fs::File;

use mount::Mount;
use router::{Router, NoRoute};
use staticfile::Static;

use lib::{Question, QuizzDataBase, QuizzDataBaseStruct, TextualContent};
#[allow(dead_code)]
mod lib;

use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct QuizzRequest {
    indice: i32,
    choice: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct QuizzEntitledResponse {
    indice: i32,
    entitled: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct QuizzReplyResponse {
    response: bool
}

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

fn root(_request: &mut Request) -> IronResult<Response> {
    let mut f = iexpect!(File::open("static/output/index.html").ok(), (status::Ok, ""));
    let mut index = String::new();
    f.read_to_string(&mut index).unwrap();
    let mut res = Response::with((status::Ok, index));
    res.headers.set(
        ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))
    );
    Ok(res)
}

fn question(_request: &mut Request) -> IronResult<Response> {
    let mut data_base = QuizzDataBaseStruct::new();
    let question = data_base.get_question();
    let response_json = QuizzEntitledResponse { indice: question.id, entitled: question.entitled };
    Ok(Response::with((status::Ok, json::encode(&response_json).unwrap())))
}

fn reply(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: QuizzRequest = json::decode(&payload).unwrap();

    let mut data_base = QuizzDataBaseStruct::new();
    let question = Question { id: request.indice, entitled: String::from(""), response: false };
    let reply = data_base.get_response(question);

    let quizz_request = QuizzRequest { indice: request.indice, choice: request.choice };

    let yes = &*TextualContent::Yes.str();

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
    }
    let response_json = QuizzReplyResponse { response: good_response };
    Ok(Response::with((status::Ok, json::encode(&response_json).unwrap())))
}

fn main() {
    let mut router = Router::new();

    router.get("/", root);
    router.post("/question", question);
    router.post("/reply", reply);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static/", Static::new(Path::new("static/output/")));

    let mut chain = Chain::new(mount);
    chain.link_after(Custom404);

    Iron::new(chain).http("localhost:3000").unwrap();
}