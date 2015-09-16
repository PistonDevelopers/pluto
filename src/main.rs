//! Redox is a server for a game competition page.

extern crate iron;

use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::headers::*;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let mut file = File::open("assets/index.html").unwrap();
        let mut index = String::new();
        file.read_to_string(&mut index).unwrap();
        let mut res = Response::with((status::Ok, index));
        res.headers.set(ContentType::html());
        Ok(res)
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
