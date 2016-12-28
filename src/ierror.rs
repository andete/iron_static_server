
use std::io;
use toml;
use daemonize::DaemonizeError;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        ParserError {
            errors: Vec<toml::ParserError>
        } {
            description("toml parser errors")
            display("parser errors: {:?}", errors)
        }
    }
    
    foreign_links {
        IoError(io::Error);
        TomlError(toml::DecodeError);
        DaemonizeError(DaemonizeError);
    }
    
}
