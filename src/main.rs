//! Redox is a server for a game competition page.

extern crate iron;

use iron::{Request, Response, IronResult, Iron, Error};
use std::io::net::ip::Ipv4Addr;
use std::path::posix::Path;

fn main() {
    fn hello(_: &mut Request) -> IronResult<Response> {
        Response::from_file(&Path::new("./assets/index.html"))
            .map_err(|e| e.erase())
    }

    Iron::new(hello).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

