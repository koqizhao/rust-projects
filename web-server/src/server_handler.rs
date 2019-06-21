
#![allow(dead_code)]

use std::sync::mpsc;
use std::net::{ TcpStream, Shutdown };
use std::marker::PhantomData;
use std::io::{ Read, Write };

use serde::{ Serialize };
use serde::de::DeserializeOwned;

macro_rules! close_return {
    ($stream: ident, $e: ident) => (
        {
            println!("Error: {}", $e);
            match $stream.shutdown(Shutdown::Both) {
                Err(e) => println!("Connection close error: {}", e),
                _ => ()
            }
            return;
        }
    );
}

pub struct WebServerHandler<Req, Res, S: Serializer, H: RequestHandler<Req, Res>> {
    sender: mpsc::Sender<TcpStream>,
    receiver: mpsc::Receiver<TcpStream>,
    serializer: S,
    handler: H,
    p1: PhantomData<Req>,
    p2: PhantomData<Res>
}

impl<Req: DeserializeOwned, Res: Serialize, S: Serializer, H: RequestHandler<Req, Res>> WebServerHandler<Req, Res, S, H> {

    pub fn new(serializer: S, handler: H) -> Self {
        let (sender, receiver) = mpsc::channel::<TcpStream>();
        WebServerHandler {
            sender,
            receiver,
            serializer,
            handler,
            p1: PhantomData,
            p2: PhantomData
        }
    }

    pub fn handle(&self, mut stream: TcpStream) {
        let mut buf: [u8; 1024] = [0u8; 1024];
        let size;
        match stream.read(&mut buf) {
            Ok(l) => size = l,
            Err(e) => close_return!(stream, e)
        }

        println!("from: {}", String::from_utf8_lossy(&buf[0..size]));

        let req = self.serializer.deserialize::<Req>(Vec::from(&buf[0..size]));
        match req {
            Ok(r) => {
                match self.handler.handle(&r) {
                    Ok(res) => {
                        match self.serializer.serialize(&res) {
                            Ok(mut buf) => {
                                match stream.write_all(&mut buf) {
                                    Err(e) => close_return!(stream, e),
                                    _ => ()
                                }

                                if let Result::Err(e) = stream.flush() {
                                    close_return!(stream, e);
                                }
                            },
                            Err(e) => close_return!(stream, e)
                        }
                    },
                    Err(e) => close_return!(stream, e)
                }
            },
            Err(e) => close_return!(stream, e)
        } 
    }

}

pub trait Serializer {

    type Error: std::error::Error;

    fn serialize<T: Serialize>(&self, v: &T) -> Result<Vec<u8>, Self::Error>;

    fn deserialize<T: DeserializeOwned>(&self, bytes: Vec<u8>) -> Result<T, Self::Error>;

}

pub trait RequestHandler<Req, Res> {

    type Error: std::error::Error;

    fn handle(&self, req: &Req) -> Result<Res, Self::Error>;

}

