extern crate pretty_env_logger;
extern crate rkv;

use rkv::{server, storage};
use std::path::PathBuf;

fn main() {
    pretty_env_logger::init().unwrap();
    let mut store = match storage::Storage::new(&PathBuf::from("/tmp")) {
        Ok(st) => st,
        Err(error) => panic!("There was a problem opening storage: {:?}", error),
    };
    server::listen(&mut store);
}
