
use std::net::TcpStream;
use std::io::{ self, Read, Error, ErrorKind };

struct HttpProtocol;

impl HttpProtocol {
    pub fn read(stream: &mut TcpStream) -> Result<String, Error> {
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
                },
                Err(err) => return Err(err)
            }
        }

        match String::from_utf8(bytes) {
            Ok(s) => {
                let parts: Vec<&str> = s.split("\r\n\r\n").collect();
                Ok("s".to_owned())
            },
            Err(err) => Err(io::Error::new(ErrorKind::InvalidData, err))
        }
    }
}