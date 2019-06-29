mod protocol;
mod server_handler;
mod server;

use server::*;
use server_handler::*;

use serde::{ Serialize };
use serde::de::DeserializeOwned;
use serde::de::Error as DError;

fn main() {
    let server_handler = WebServerHandler::new(MySerializer, MyRequestHandler);
    let mut server = WebServer::new("localhost".to_string(), 8080, server_handler);
    println!("Server is starting: {}:{}", server.host(), server.port());
    match server.start() {
        Some(err) => println!("Server error: {}", err),
        None => ()
    }
}

struct MyRequestHandler;

impl RequestHandler<String, String> for MyRequestHandler {

    type Error = std::io::Error;

    fn handle(&self, req: &String) -> Result<String, Self::Error> {
        Ok(req.clone())
    }

}


struct MySerializer;

impl Serializer for MySerializer {

    type Error = serde_json::error::Error;

    fn serialize<T: Serialize>(&self, v: &T) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_string(v)?.into_bytes())
    }

    fn deserialize<T: DeserializeOwned>(&self, bytes: Vec<u8>) -> Result<T, Self::Error> {
        match serde_json::from_slice::<T>(&bytes) {
            Ok(t) => Ok(t),
            Err(e) => Err(serde_json::error::Error::custom(e))
        }
    }

}