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
#[macro_use]
extern crate log;
extern crate env_logger;

use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

fn load_config(filename:&str) -> Result<Config> {
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
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/server.toml");
    let config = load_config(&filename)?;
    println!("{:?}", config);
    let mut vhost_h = HashMap::new();
    // create a Vhosts per port we're listening on
    for (name, _listen) in &config.listen {
        let vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::InternalServerError, "vhost"))));
        vhost_h.insert(name, vhosts);
    }
    // for each vhost add it to the Vhosts for the used listening address
    for (_name, vhost) in &config.vhost {
        if let Some(mut vhosts) = vhost_h.get_mut(&vhost.listen) {
            let name = &vhost.hostname;
            if let Some(ref static_files) = vhost.static_files {
                make_static(&mut vhosts, name, static_files);
                println!("static {} on {}", name, vhost.listen);
            }
        }
    }
    let mut children = vec![];
    for (name, listen) in &config.listen {
        println!("{:?}", (name, listen));
        if let Some(vhosts) = vhost_h.remove(&name) {
            let address = listen.address.as_str();
            let iron = Iron::new(vhosts);
            let listener = match listen.tls {
                None => {
                    println!("http on {}", address);
                    iron.http(address)
                },
                Some(ref tls) => {
                    let cert = PathBuf::from(&tls.cert);
                    let key = PathBuf::from(&tls.key);
                    println!("https on {}", address);
                    iron.https(address, cert, key)
                },
            };
            children.push(std::thread::spawn(move || { listener.unwrap() }));
        }
    }

    for child in children {
        child.join().unwrap(); // TODO
    }
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

mod config;
mod ierror;
