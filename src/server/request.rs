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

        let argc = line_iter
            .next()
            .and_then(|arg| arg.to_digit(10))
            .ok_or(io::Error::new(io::ErrorKind::Other, "Malformed request"))?;

        let mut args: Vec<Vec<u8>> = Vec::new();

        for _i in 0..argc {
            read_line(buf)?
                .and_then(|line| {
                    if line.starts_with('$') {
                        Some(line)
                    } else {
                        None
                    }
                })
                .and_then(|line| {
                    let max_slice_size = if line.len() > 2 { 3 } else { 2 };
                    line.get(1..max_slice_size)?.parse().ok()
                })
                .and_then(|size: u64| {
                    let mut buffer: Vec<u8> = vec![];
                    buf.take(size + 2).read_to_end(&mut buffer).ok();
                    args.push(buffer[..buffer.len() - 2].to_vec());
                    Some(0)
                })
                .ok_or(io::Error::new(io::ErrorKind::Other, "Malformed request"))?;
        }

        match &args[0][..] {
            b"PING" => Ok(Some(Command::PING)),
            b"SET" => {
                if args.len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                }
                Ok(Some(Command::SET(args.swap_remove(2))))
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
