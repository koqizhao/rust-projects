use std::net::*;
use std::io::Error;
use std::thread;
use std::time;
use std::marker::PhantomData;

use serde::{ Serialize };
use serde::de::DeserializeOwned;

use crate::protocol::*;
use crate::serializer::*;
use super::server_handler::*;

pub struct WebServer<P: Protocol, S: Serializer, Req, Res, H: RequestHandler<Req, Res>> {
    host: String,
    port: u16,
    handler: WebServerHandler<P, S, Req, Res, H>,
    listener: Option<TcpListener>,
    p1: PhantomData<Req>,
    p2: PhantomData<Res>
}

impl<P: Protocol, S: Serializer, Req: DeserializeOwned + Default, Res: Serialize, H: RequestHandler<Req, Res>>
    WebServer<P, S, Req, Res, H> {

    pub fn new(host: String, port: u16, handler: WebServerHandler<P, S, Req, Res, H>) -> Self {
        WebServer {
            host,
            port,
            listener: None,
            handler,
            p1: PhantomData,
            p2: PhantomData
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub fn start(&mut self) -> Option<Error> {
        match &self.listener {
            Some(_) => {
                self.listener = None;
            },
            _ => ()
        }

        let addr = self.host.clone() + ":" + &self.port.to_string();
        let band_result = TcpListener::bind(addr);
        if let Err(err) = band_result {
            return Some(err);
        }

        self.listener = Some(band_result.unwrap());

        self.accept();

        None
    }

    fn listener(&self) -> &TcpListener {
        self.listener.as_ref().unwrap()
    }

    fn accept(&self) {
        loop {
            match self.listener().accept() {
                Ok((stream, _)) => {
                    self.handler.handle(stream);
                },
                Err(err) => {
                    println!("Accept request error: {}", err);
                    thread::sleep(time::Duration::from_millis(10));
                }
            }
        }
    }
}