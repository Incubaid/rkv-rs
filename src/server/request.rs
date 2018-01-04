use Command;
use hex;

use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;

pub fn decode(buf: &mut BufReader<&TcpStream>) -> io::Result<Option<Command>> {
    if let Some(line) = read_line(buf)? {
        let mut line_iter = line.chars();

        if line_iter.next() != Some('*') {
            return Err(io::Error::new(io::ErrorKind::Other, "not an array"));
        }

        let argc = match line_iter.next() {
            Some(arg) => arg.to_digit(10),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to parse request",
                ));
            }
        };

        let mut args: Vec<Vec<u8>> = Vec::new();

        if let Some(argc) = argc {
            for _i in 0..argc {
                if let Some(mut line) = read_line(buf)? {
                    if !line.starts_with('$') {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Malformed request (string)",
                        ));
                    }

                    let max_slice_size = if line.len() > 2 { 3 } else { 2 };
                    let size: u64 = match line.get(1..max_slice_size) {
                        Some(s) => s.parse().map_err(|_| {
                            io::Error::new(io::ErrorKind::Other, "Malformed request")
                        })?,
                        None => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                        }
                    };

                    let mut buffer: Vec<u8> = vec![];
                    buf.take(size + 2).read_to_end(&mut buffer)?;
                    args.push(buffer[..buffer.len() - 2].to_vec());
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "failed to parse request",
                    ));
                }
            }
        }

        match &args[0][..] {
            b"PING" => Ok(Some(Command::PING)),
            b"SET" => {
                if args.len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                }
                Ok(Some(Command::SET(args[2].clone())))
            }
            b"GET" => {
                if args.len() < 1 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                }

                let hash = match hex::decode(&args[1]) {
                    Ok(hash) => hash,
                    Err(_) => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                    }
                };
                Ok(Some(Command::GET(hash)))
            }
            _ => Ok(Some(Command::NOTSUPPORTED)),
        }
    } else {
        Ok(None)
    }
}

fn read_line(buf: &mut BufReader<&TcpStream>) -> io::Result<Option<(String)>> {
    let mut line = String::new();
    buf.read_line(&mut line)?;

    if !line.is_empty() {
        Ok(Some(line.as_str().trim().to_owned()))
    } else {
        Ok(None)
    }
}
