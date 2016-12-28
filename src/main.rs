extern crate iron_static_server;
#[macro_use]
extern crate log;
extern crate syslog;

use std::env;

use syslog::Facility;

fn main() {
    let syslog = syslog::unix(Facility::LOG_USER).unwrap();
    log::set_logger(|max_level| {
        max_level.set(log::LogLevelFilter::Info);
        syslog
    }).unwrap();
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {:?} <configfile.toml>", args[0]);
        return;
    }
    
    let filename = &args[1];

    if let Err(ref e) = iron_static_server::run(filename) {
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
