use std::net::TcpListener;

use std::io::{BufReader, BufWriter, Result, Write};
use std::net::TcpStream;

use Command;
use storage::Storage;

mod request;

fn handle(stream: &TcpStream, storage: &mut Storage) -> Result<()> {
    let mut reader = BufReader::new(stream); // TODO: handle properly
    let mut writer = BufWriter::new(stream);

    loop {
        let x = request::decode(&mut reader);

        if let Some(payload) = x.unwrap() {
            match payload {
                Command::PING => write(&mut writer, b"+PONG\r\n")?,
                Command::SET(val) => match storage.set(&val) {
                    Ok(hash) => {
                        let msg = format!("+{}\r\n", hash);
                        write(&mut writer, msg.as_bytes())?
                    }
                    Err(error) => {
                        let msg = format!("-Error: {}", error);
                        write(&mut writer, msg.as_bytes())?
                    }
                },
                Command::GET(hash) => match storage.get(&hash) {
                    Ok(val) => {
                        // $xx\r\n + payload + \r\n
                        let msg = format!("${}\r\n", val.len());
                        let vec = [msg.as_bytes(), &val, b"\r\n"].concat();
                        write(&mut writer, &vec)?
                    }
                    Err(error) => {
                        let msg = format!("-Error: {}", error);
                        write(&mut writer, msg.as_bytes())?
                    }
                },
                Command::NOTSUPPORTED => write(&mut writer, b"-Command not handled\r\n")?,
            }; // handle properly

            writer.flush()?;
        } else {
            break;
        }
    }
    Ok(())
}

fn write<W: Write>(writer: &mut BufWriter<W>, data: &[u8]) -> Result<()> {
    assert!(
        writer.write(data)? > 1,
        "Something wrong happened while writing to buffer"
    );
    Ok(())
}

pub fn listen(storage: &mut Storage) {
    let listener = TcpListener::bind("0.0.0.0:9900").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(&stream, storage).unwrap(); // TODO: handle error
    }
}
