extern crate iron_static_server;
#[macro_use]
extern crate log;
extern crate syslog;
extern crate argparse;

use syslog::Facility;
use argparse::{ArgumentParser, StoreTrue, Store, StoreOption};

fn main() {
    
    // set up logging
    let syslog = syslog::unix(Facility::LOG_USER).unwrap();
    log::set_logger(|max_level| {
        max_level.set(log::LogLevelFilter::Info);
        syslog
    }).unwrap();
    info!("starting");

    let mut daemonize = false;
    let mut filename = String::new();
    let mut username:Option<String> = None;
    {
    let mut ap = ArgumentParser::new();
    ap.set_description("Run the iron based static vhosts/TLS web server");
    ap.refer(&mut daemonize)
            .add_option(&["-d", "--deamonize"], StoreTrue,
                        "deamonize the process");
    ap.refer(&mut username)
            .add_option(&["-u", "--username"], StoreOption,
                        "drop privileges to user");
    ap.refer(&mut filename)
        .add_argument("filename", Store,
                      "toml configuration file")
            .required();
    
        ap.parse_args_or_exit();
    }
    
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
