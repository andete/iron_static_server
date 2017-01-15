extern crate iron_static_server;
#[macro_use]
extern crate log;
extern crate syslog;
extern crate clap;

use syslog::Facility;
use clap::{Arg, App};

fn main() {
    
    // set up logging
    let syslog = syslog::unix(Facility::LOG_USER).unwrap();
    log::set_logger(|max_level| {
        max_level.set(log::LogLevelFilter::Info);
        syslog
    }).unwrap();
    info!("starting");

    let matches = App::new("Iron Static Server")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Joost Yervante Damad <joost@damad.be>")
                          .about("Serve multiple static websites.")
                          .arg(Arg::with_name("DAEMONIZE")
                               .short("d")
                               .long("daemonize")
                               .required(false)
                               .help("daemonize the process"))
                          .arg(Arg::with_name("USER")
                               .short("u")
                               .long("username")
                               .required(false)
                               .value_name("USERNAME")
                               .takes_value(true)
                               .help("drop privileges to user"))
                          .arg(Arg::with_name("FILENAME")
                               .required(true)
                               .index(1)
                               .help("the configuration filename"))
                          .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    let daemonize = matches.is_present("DAEMONIZE");
    let username = matches.value_of("USERNAME");
    
    if let Err(ref e) = iron_static_server::run(&filename, daemonize, username) {
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
