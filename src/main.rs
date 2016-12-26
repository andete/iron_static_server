#![feature(proc_macro)]

extern crate iron;
extern crate iron_vhosts;
extern crate staticfile;
extern crate mount;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use iron_vhosts::Vhosts;
use iron::status;

use config::Config;
use ierror::*;

fn read_file(name: &str) -> Result<String> {
    let mut f = File::open(name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn load_config() -> Result<Config> {
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/server.toml");
    let config_str = read_file(&filename)?;
    let mut parser = toml::Parser::new(&config_str);
    let parsed = match parser.parse() {
        Some(x) => Ok(x),
        None => Err(ErrorKind::ParserError { errors:parser.errors }),
    }?;
    let value = toml::Value::Table(parsed);
    println!("value: {:?}", value);
    let mut decoder = toml::Decoder::new(value);
    let config:Config = serde::Deserialize::deserialize(&mut decoder)?;
    println!("config: {:?}", config);
    Ok(config)
}

fn make_static(vhosts: &mut Vhosts, hostname:&str, path:&str) {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(path)));
    vhosts.add_host(hostname, mount);
}

fn run() -> Result<()> {
    let config = load_config()?;
    println!("{:?}", config);
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::InternalServerError, "vhost"))));
    for (_name, vhost) in &config.vhost {
        let name = &vhost.hostname;
        if let Some(ref static_files) = vhost.static_files {
            make_static(&mut vhosts, name, static_files);
        }
    }

    let child = std::thread::spawn(move || {
        //Iron::new(vhosts).http(config.listen.as_str()).unwrap(); // TODO remove unwrap :)
    });
    child.join().unwrap(); // TODO
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

mod config;
mod ierror;
