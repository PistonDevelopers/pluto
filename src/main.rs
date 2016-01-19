extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate logger;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use rustc_serialize::json::{ToJson, Json};

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

struct User {
    name: String,
    age: u16,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();

        map.insert("name".to_string(), self.name.to_json());
        map.insert("age".to_string(), self.age.to_json());
        map.to_json()
    }
}

fn test(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = User {
        name: "Adam".to_string(),
        age: 32u16,
    };
    resp.set_mut(Template::new("test", data)).set_mut(status::Ok);
     Ok(resp)
}

fn main() {
    let mut bars = HandlebarsEngine::new2();
    bars.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    if let Err(r) = bars.reload() {
        panic!("{:?}", r);
    }

    let mut router = Router::new();
    router.
        get("/", home).
        get("/rules", rules).
        get("/games", games).
        get("/test", test);

    let mut mount = Mount::new();
    mount.
        mount("/", router).
        mount("/css", Static::new("public/css")).
        mount("/img", Static::new("public/img"));

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link_after(bars);

    let url = "localhost:3000";
    match Iron::new(chain).http(url) {
        Ok(_) => println!("Pluto running on http://{}", url),
        Err(e) => println!("Pluto failed to run. Error: {}", e)
    };
}
