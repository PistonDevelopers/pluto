extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate logger;

use std::io::prelude::*;
use std::fs::File;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;

fn load_html_from_file(file: &str) -> Option<String> {
    let mut f = File::open("templates/".to_string() + file).unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Some(s),
        Err(_) => None
    }
}

fn home(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let text_html = load_html_from_file("index.html");
    match text_html {
        Some(s) => Ok(Response::with((content_type, status::Ok, s))),
        None => Ok(Response::with(status::NotFound))
    }
}

fn rules(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let text_html = load_html_from_file("rules.html");
    match text_html {
        Some(s) => Ok(Response::with((content_type, status::Ok, s))),
        None => Ok(Response::with(status::NotFound))
    }
}

fn games(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let text_html = load_html_from_file("games.html");
    match text_html {
        Some(s) => Ok(Response::with((content_type, status::Ok, s))),
        None => Ok(Response::with(status::NotFound))
    }
}

fn main() {
    let mut router = Router::new();
    router.
        get("/", home).
        get("/rules", rules).
        get("/games", games);

    let mut mount = Mount::new();
    mount.
        mount("/", router).
        mount("/css", Static::new("public/css")).
        mount("/img", Static::new("public/img"));

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let url = "localhost:3000";
    match Iron::new(chain).http(url) {
        Ok(_) => println!("Redox running on http://{}", url),
        Err(e) => println!("Redox failed to run. Error: {}", e)
    };
}
