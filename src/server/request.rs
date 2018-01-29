use Command;
use hex;

use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use atoi::atoi;

pub struct Request<'a> {
    pub reader: BufReader<&'a TcpStream>,
}

impl<'a> Request<'a> {
    // add code here
    pub fn decode(&mut self) -> io::Result<Option<Command>> {
        if let Some(line) = self.read_line()? {
            let mut line_iter = line.iter();

            if line_iter.next() != Some(&b'*') {
                return Err(io::Error::new(io::ErrorKind::Other, "not an array"));
            }

            let argc = line_iter
                .next()
                .and_then(|arg| atoi::<u8>(&[*arg]))
                .ok_or(io::Error::new(io::ErrorKind::Other, "Malformed request"))?;

            let mut args: Vec<Vec<u8>> = Vec::new();

            for _i in 0..argc {
                self.read_line()?
                    .and_then(|line| {
                        if line.starts_with(&[b'$']) {
                            Some(line)
                        } else {
                            None
                        }
                    })
                    .and_then(|line| {
                        atoi::<usize>(line.get(1..)?)
                    })
                    .and_then(|size: usize| {
                        let mut line = self.read_exact(size + 2).ok()??;
                        line.truncate(size);
                        args.push(line);
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

    fn read_line(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut line = vec![];
        self.reader.read_until(b'\n', &mut line)?;
        line.pop();
        line.pop();
        if !line.is_empty() {
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }

    fn read_exact(&mut self, size: usize) -> io::Result<Option<Vec<u8>>> {
        let mut line = vec![0u8; size];
        self.reader.read_exact(&mut line)?;

        if !line.is_empty() {
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }
}
