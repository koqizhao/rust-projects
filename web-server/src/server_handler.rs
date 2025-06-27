use std::marker::PhantomData;
use std::net::{Shutdown, TcpStream};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::protocol::Protocol;
use crate::serializer::Serializer;

macro_rules! close_return {
    ($stream: ident, $e: ident) => {{
        println!("Error: {}", $e);
        match $stream.shutdown(Shutdown::Both) {
            Err(e) => println!("Connection close error: {}", e),
            _ => (),
        }
        return;
    }};
}

pub trait ServerHandler {
    fn handle(&self, stream: TcpStream);
}

pub trait RequestHandler<Req, Res> {
    type Error: std::error::Error;

    fn handle(&self, req: &Req) -> Result<Res, Self::Error>;
}

pub struct WebServerHandler<P: Protocol, S: Serializer, Req, Res, H: RequestHandler<Req, Res>> {
    protocol: P,
    serializer: S,
    handler: H,
    p1: PhantomData<Req>,
    p2: PhantomData<Res>,
}

impl<
    P: Protocol,
    S: Serializer,
    Req: DeserializeOwned + Default,
    Res: Serialize,
    H: RequestHandler<Req, Res>,
> WebServerHandler<P, S, Req, Res, H>
{
    pub fn new(protocol: P, serializer: S, handler: H) -> Self {
        WebServerHandler {
            protocol,
            serializer,
            handler,
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<
    P: Protocol,
    S: Serializer,
    Req: DeserializeOwned + Default,
    Res: Serialize,
    H: RequestHandler<Req, Res>,
> ServerHandler for WebServerHandler<P, S, Req, Res, H>
{
    fn handle(&self, mut stream: TcpStream) {
        let input = match self.protocol.read(&mut stream) {
            Ok(s) => s,
            Err(err) => close_return!(stream, err),
        };

        match self.serializer.deserialize::<Req>(&input) {
            Ok(r) => match self.handler.handle(&r) {
                Ok(res) => match self.serializer.serialize(&res) {
                    Ok(s) => match self.protocol.write(&mut stream, s.as_bytes()) {
                        Err(e) => close_return!(stream, e),
                        _ => (),
                    },
                    Err(e) => close_return!(stream, e),
                },
                Err(e) => close_return!(stream, e),
            },
            Err(e) => close_return!(stream, e),
        }
    }
}
