use std::io::{self, Error, ErrorKind, Read, Write};
use std::net::TcpStream;

pub trait Protocol {
    fn read(&self, stream: &mut TcpStream) -> Result<String, Error>;
    fn write(&self, stream: &mut TcpStream, bytes: &[u8]) -> Result<(), Error>;
}

const HEADER_BODY_SPLITTER: &str = "\r\n\r\n";
const NO_BODY: &str = "";
const OK_LINE: &str = "HTTP/1.1 200 OK\r\n";
const EMPTY_LINE: &str = "\r\n";

pub struct HttpProtocol;

impl Protocol for HttpProtocol {
    fn read(&self, stream: &mut TcpStream) -> Result<String, Error> {
        let mut buf = [0u8; 1024];
        let mut bytes = Vec::<u8>::new();
        loop {
            match stream.read(&mut buf) {
                Ok(size) => {
                    if size > 0 {
                        bytes.extend_from_slice(&buf[0..size]);
                    }

                    if size == 0 || size < buf.len() {
                        break;
                    }
                }
                Err(err) => return Err(err),
            }
        }

        let err: Error = match String::from_utf8(bytes) {
            Ok(s) => {
                let parts: Vec<&str> = s.split(HEADER_BODY_SPLITTER).collect();
                match parts.len() {
                    0 => io::Error::new(ErrorKind::InvalidData, s.to_owned()),
                    1 => return Ok(NO_BODY.to_owned()),
                    _ => return Ok(parts[1].to_owned()),
                }
            }
            Err(err) => io::Error::new(ErrorKind::InvalidData, err),
        };
        Err(err)
    }

    fn write(&self, stream: &mut TcpStream, bytes: &[u8]) -> Result<(), Error> {
        stream.write_all(OK_LINE.as_bytes())?;
        let content_length_line = format!("Content-Length: {}\r\n", bytes.len());
        stream.write_all(content_length_line.as_bytes())?;
        stream.write_all(EMPTY_LINE.as_bytes())?;
        stream.write_all(bytes)?;
        stream.flush()?;
        Ok(())
    }
}
