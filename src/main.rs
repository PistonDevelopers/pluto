//! Redox is a server for a game competition page.

extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.
        mount("/", Static::new("public/")).
        mount("/css", Static::new("public/css")).
        mount("/img", Static::new("public/img"));

    let url = "localhost:3000";
    Iron::new(mount).http(url).unwrap();
    println!("Redox running on http://{}", url);
}
