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

fn home(req: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut f = File::open("templates/index.html").unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(Response::with((content_type, status::Ok, s))),
        Err(_) => Ok(Response::with(status::NotFound))
    }
}

fn rules(req: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut f = File::open("templates/rules.html").unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(Response::with((content_type, status::Ok, s))),
        Err(_) => Ok(Response::with(status::NotFound))
    }
}

fn games(req: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut f = File::open("templates/games.html").unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(Response::with((content_type, status::Ok, s))),
        Err(_) => Ok(Response::with(status::NotFound))
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
