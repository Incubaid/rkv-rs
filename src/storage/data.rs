use openssl::sha::sha256;
use byteorder::{LittleEndian, WriteBytesExt};
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result, SeekFrom};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use super::Metadata;

pub struct Data {
    handler: File,
}

impl Data {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .mode(0o600)
            .append(true)
            .open(path)?;

        if file.write(b"X")? != 1 {
            return Err(Error::new(
                ErrorKind::Interrupted,
                "Error: Data corruption may have occurred",
            ));
        }

        Ok(Data { handler: file })
    }

    pub fn insert(&mut self, data: &[u8]) -> Result<Metadata> {
        // buf representation => |hash|data.len|data|
        let mut buf = Vec::new();

        let metadata = Metadata {
            offset: (&self.handler).seek(SeekFrom::Current(0))?,
            length: data.len() as u32,
            hash: sha256(&data[..]),
        };

        buf.extend_from_slice(&metadata.hash);

        buf.reserve(metadata.length.to_string().len());
        buf.write_u32::<LittleEndian>(metadata.length)?;

        buf.extend_from_slice(&data[..]);

        assert!(
            self.handler.write(&buf[..])? > 1,
            "Something wrong happened while writing to rkv-data file"
        );

        Ok(metadata)
    }

    pub fn fetch(&mut self, val: &Metadata) -> Result<Vec<u8>> {
        // hash|content.len()
        //  32 |       4      = 36
        self.handler.seek(SeekFrom::Start(val.offset + 36))?;
        let mut buffer = vec![0u8; val.length as usize];
        self.handler.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storage::test_utils::{run_test, TEST_DIR};
    use std::fs::File;

    fn new_data() -> Data {
        let path = PathBuf::from(TEST_DIR).join("rkv-data");
        Data::new(path).unwrap()
    }

    fn read_data() -> Vec<u8> {
        let path = PathBuf::from(TEST_DIR).join("rkv-data");
        let mut f = File::open(path).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn data_new_single_time() {
        run_test(|| {
            new_data();
            assert_eq!(&read_data(), b"X");
        })
    }

    #[test]
    fn data_new_multiple_times() {
        run_test(|| {
            new_data();
            new_data();
            assert_eq!(&read_data(), b"XX");
        })
    }

    #[test]
    fn data_insert_single_times() {
        run_test(|| {
            let mut data = new_data();
            let content = b"testdata";
            data.insert(content).unwrap();

            // X|hash|content.len()|content|
            // 1| 32 |       4     |   8   | = 45
            assert_eq!(read_data().len(), 45);
        })
    }

    #[test]
    fn data_insert_multiple_times() {
        run_test(|| {
            let mut data = new_data();
            let content = b"testdata";
            data.insert(content).unwrap();
            data.insert(content).unwrap();

            // X|hash|content.len()|content|hash|content.len()|content|
            // 1| 32 |       4     |   8   | 32 |       4     |   8   | = 1 + 44 * 2
            assert_eq!(read_data().len(), 89);
        })
    }

    #[test]
    fn data_fetch_single_insert_single() {
        run_test(|| {
            let mut data = new_data();
            let content = b"testdata";
            let metadata = data.insert(content).unwrap();

            let val = data.fetch(&metadata).unwrap();

            assert_eq!(val, content);
        })
    }

    #[test]
    fn data_fetch_multiple_insert_single() {
        run_test(|| {
            let mut data = new_data();
            let content = b"testdata";
            let metadata = data.insert(content).unwrap();

            let val = data.fetch(&metadata).unwrap();
            assert_eq!(val, content);
            let val = data.fetch(&metadata).unwrap();
            assert_eq!(val, content);
        })
    }
    #[test]
    fn data_fetch_multiple_insert_multiple() {
        run_test(|| {
            let mut data = new_data();
            let content1 = b"testdata";
            let metadata1 = data.insert(content1).unwrap();
            let content2 = b"version2data";
            let metadata2 = data.insert(content2).unwrap();

            let val = data.fetch(&metadata1).unwrap();
            assert_eq!(val, content1);
            let val = data.fetch(&metadata1).unwrap();
            assert_eq!(val, content1);

            let val = data.fetch(&metadata2).unwrap();
            assert_eq!(val, content2);
            let val = data.fetch(&metadata2).unwrap();
            assert_eq!(val, content2);
        })
    }

}
