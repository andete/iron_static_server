#![feature(proc_macro)]

extern crate iron;
extern crate iron_vhosts;
extern crate staticfile;
extern crate mount;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use std::path::Path;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use iron_vhosts::Vhosts;
use iron::status;

fn media_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "media")))
}

fn make_static(vhosts: &mut Vhosts, hostname:&str, path:&str) {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(path)));
    vhosts.add_host(hostname, mount);
}

fn main() {
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));

    make_static(&mut vhosts, "localhost", "/tmp/foo");
    //Add any host specific handlers
    vhosts.add_host("media.localhost", media_handler);

    Iron::new(vhosts).http("127.0.0.1:3000").unwrap();
}

mod config;
