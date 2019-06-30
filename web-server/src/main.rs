#![allow(dead_code)]

mod protocol;
mod serializer;
mod server_handler;
mod server;
mod service;

use protocol::*;
use serializer::*;
use server::*;
use server_handler::*;
use service::*;

fn main() {
    let server_handler = WebServerHandler::new(HttpProtocol, JsonSerializer, MyRequestHandler);
    let mut server = WebServer::new("localhost".to_string(), 8080, server_handler);
    println!("Server is starting: {}:{}", server.host(), server.port());
    match server.start() {
        Some(err) => println!("Server error: {}", err),
        None => ()
    }
}

