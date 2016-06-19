extern crate tera;

use std::io::prelude::*;
use std::fs::File;

use tera::Tera;
use tera::Context;

use lib::TextualContent;

#[allow(dead_code)]
mod lib;

fn main() {
    let tera = Tera::new("static/templates/**/*.html");

    let mut context = Context::new();
    context.add("title", &TextualContent::Title.str());
    context.add("yes", &TextualContent::Yes.str());
    context.add("no", &TextualContent::No.str());
    context.add("unchecked", &TextualContent::UnChecked.str());
    context.add("checked", &TextualContent::Checked.str());

    let rendered = tera.render("index.html", context).unwrap();
    let mut file = File::create("static/output/index.html").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
