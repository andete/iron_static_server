extern crate iron_static_server;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

fn main() {
    env_logger::init().unwrap();
    // change to the examples directory, to make the relative paths
    // contained in the example config work
    let mut dirname = String::new();
    dirname.push_str(env!("CARGO_MANIFEST_DIR"));
    dirname.push_str("/examples");
    env::set_current_dir(&dirname).unwrap();
    let filename = "server.toml";
    if let Err(ref e) = iron_static_server::run(&filename) {
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
