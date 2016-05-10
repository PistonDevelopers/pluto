extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate logger;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use rustc_serialize::json::{ToJson, Json};

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", String::new())).set_mut(status::Ok);
    Ok(resp)
}

fn rules(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("rules", String::new())).set_mut(status::Ok);
    Ok(resp)
}

fn games(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("games", String::new())).set_mut(status::Ok);
    Ok(resp)
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
    let data = User {
        name: "Adam".to_string(),
        age: 32u16,
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("test", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut bars = HandlebarsEngine::new();
    bars.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    if let Err(r) = bars.reload() {
        panic!("{:?}", r);
    }

    let mut router = Router::new();
    router.
        get("/", index).
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
