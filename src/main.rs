extern crate clap;
extern crate env_logger;
extern crate rkv;

use rkv::{server, storage};
use clap::{App, Arg};
use std::path::PathBuf;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let matches = App::new("rkv")
        .version("0.1.0")
        .about("Redis protocol based always append key-value store")
        .arg(
            Arg::with_name("dir")
                .long("dir")
                .value_name("DIR")
                .help("backend directory (default /tmp)\n")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listen")
                .long("listen")
                .value_name("ADDR")
                .help("listen address (default 0.0.0.0:9900)\n")
                .takes_value(true),
        )
        .get_matches();


    let dir = matches.value_of("dir").unwrap_or("/tmp");
    let addr = matches.value_of("listen").unwrap_or("0.0.0.0:9900");
    let mut store = match storage::Storage::new(&PathBuf::from(dir)) {
        Ok(st) => st,
        Err(error) => panic!("There was a problem opening storage: {:?}", error),
    };
    server::listen(addr, &mut store);
}
