use std::fs::{File, OpenOptions};
use std::{io, str};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use super::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};
use bincode::{deserialize_from, serialize_into, Bounded, ErrorKind, Infinite};
use hex;

const HEADER_SIZE: u64 = 24;
const BRANCH_NUMBER: usize = 256;

pub struct Index {
    handler: File,
    branches: Vec<Vec<Metadata>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    magic: [u8; 4],
    version: u32,
    created: u64,
    opened: u64,
}

impl Index {
    pub fn new(path: PathBuf) -> io::Result<Index> {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .mode(0o600)
            .open(path)?;

        let mut index = Index {
            handler: file,
            branches: vec![Vec::new(); BRANCH_NUMBER],
        };

        index.load()?;

        Ok(index)
    }

    pub fn insert(&mut self, entry: Metadata) -> io::Result<String> {
        if let Some(entr) = self.insert_mem(&entry)? {
            serialize_into(&mut (&self.handler), &entr, Infinite).map_err(|e| {
                let msg = format!("Failed to write header to index file: {:?}", e);
                io::Error::new(io::ErrorKind::Interrupted, msg)
            })?;
        } else {
            // TODO: Add debug here
        }
        Ok(hex::encode(entry.hash))
    }

    pub fn get(&self, hash: [u8; 32]) -> Option<&Metadata> {
        for entry in &self.branches[hash[0] as usize] {
            if hash == entry.hash {
                return Some(entry);
            }
        }
        None
    }
    fn insert_mem<'a>(&mut self, entry: &'a Metadata) -> io::Result<Option<&'a Metadata>> {
        if self.get(entry.hash).is_some() {
            return Ok(None);
        }

        self.branches[entry.hash[0] as usize].push(entry.clone());
        Ok(Some(entry))
    }

    fn load(&mut self) -> io::Result<()> {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut header: Header = deserialize_from(&mut (&self.handler), Bounded(HEADER_SIZE))
            .unwrap_or_else(|e| match *e.as_ref() {
                ErrorKind::Io(_) => Header {
                    magic: *b"IDXO",
                    version: 1,
                    created: epoch,
                    opened: epoch,
                },
                _ => {
                    println!("{}", e); // TODO: replace with logging
                    panic!("Corrupted index file");
                }
            });

        self.handler.seek(io::SeekFrom::Start(0))?;
        header.opened = epoch;
        serialize_into(&mut (&self.handler), &header, Bounded(HEADER_SIZE)).map_err(|e| {
            let msg = format!("Failed to write header to index file: {:?}", e);
            io::Error::new(io::ErrorKind::Interrupted, msg)
        })?;

        self.handler.seek(io::SeekFrom::Start(HEADER_SIZE))?;
        while let Ok(entry) = deserialize_from(&mut self.handler, Bounded(44)) {
            self.insert_mem(&entry)?;
        }

        self.dump();
        Ok(())
    }

    fn dump(&self) {
        for entry_chain in &self.branches {
            for entry in entry_chain.iter() {
                println!("hash: {:?}", hex::encode(entry.hash));
                println!("offset: {}", entry.offset);
                println!("size: {}", entry.length);
                println!("-----------");
            }
        }
    }
}
