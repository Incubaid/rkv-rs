extern crate bincode;
extern crate byteorder;
extern crate hex;
extern crate openssl;
#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
pub enum Command {
    PING,
    SET(Vec<u8>),
    GET(Vec<u8>),
    NOTSUPPORTED,
}

pub mod server;
pub mod storage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
