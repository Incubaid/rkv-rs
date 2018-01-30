extern crate atoi;
extern crate bincode;
extern crate byteorder;
extern crate hex;
extern crate openssl;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

pub enum Command {
    PING,
    SET(Vec<u8>),
    GET(Vec<u8>),
    NOTSUPPORTED,
}

pub mod server;
pub mod storage;
