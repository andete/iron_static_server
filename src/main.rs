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

fn localhost_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "localhost")))
}

fn media_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "media")))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("/tmp/foo")));

    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));

    //Add any host specific handlers
    vhosts.add_host("localhost", localhost_handler);
    vhosts.add_host("media.localhost", media_handler);

    Iron::new(vhosts).http("127.0.0.1:3000").unwrap();
}

mod config;
