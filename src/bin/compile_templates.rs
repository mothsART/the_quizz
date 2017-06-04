extern crate the_quizz;
extern crate tera;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;

use tera::Tera;
use tera::Context;


use the_quizz::*;

fn main() {
    let tera = Tera::new("static/templates/**/*.html");

    let content = textual_content(Some(String::from("en_US")));
    let langs = langs();

    let mut context = Context::new();
    context.add("title", &content.title);
    context.add("yes", &content.yes);
    context.add("no", &content.no);
    context.add("unchecked", &content.unchecked);
    context.add("checked", &content.checked);

    let rendered = tera.render("index.html", context).unwrap();
    let mut file = File::create("static/output/index.html").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
