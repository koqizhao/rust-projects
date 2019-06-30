use std::net::*;
use std::io::Error;
use std::thread;
use std::time;
use std::sync::Arc;

use super::server_handler::*;

pub struct WebServer<T: ServerHandler> {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
    handler: Arc<T>,
}

impl<T: ServerHandler + Send + Sync + 'static> WebServer<T> {

    pub fn new(host: String, port: u16, handler: T) -> Self {
        WebServer {
            host,
            port,
            listener: None,
            handler: Arc::new(handler)
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
                    let h = self.handler.clone();
                    thread::spawn(move || {
                        h.handle(stream);
                    });
                },
                Err(err) => {
                    println!("Accept request error: {}", err);
                    thread::sleep(time::Duration::from_millis(10));
                }
            }
        }
    }
}