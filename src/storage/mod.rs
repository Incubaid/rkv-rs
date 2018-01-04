use std::path::PathBuf;
use std::io;

mod data;
mod index;
mod test_utils;

use self::data::*;
use self::index::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    pub offset: u64,
    pub length: u32,
    pub hash: [u8; 32],
}

pub struct Storage {
    data: Data,
    index: Index,
}

impl Storage {
    // TODO: consider asRef here as the case in the std File::create
    pub fn new(dir: &PathBuf) -> io::Result<Self> {
        Ok(Self {
            data: Data::new(dir.join("rkv-data"))?,
            index: Index::new(dir.join("rkv-index"))?,
        })
    }

    pub fn set(&mut self, val: &[u8]) -> io::Result<String> {
        let metadata = self.data.insert(val)?;
        self.index.insert(metadata)
    }

    pub fn get(&mut self, val: &[u8]) -> io::Result<Vec<u8>> {
        let mut array = [0u8; 32];
        for (&x, p) in val.iter().zip(array.iter_mut()) {
            *p = x;
        }

        if let Some(r) = self.index.get(array) {
            return Ok(self.data.fetch(r)?);
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Value is not found"));
        }
    }
}
