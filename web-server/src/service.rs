use serde::{ Serialize, Deserialize };

use crate::server_handler::RequestHandler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String
}

impl Default for Request {
    fn default() -> Self {
        Request {
            name: "World".to_owned()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub welcome: String
}

impl Default for Response {
    fn default() -> Self {
        Response {
            welcome: "Hello, World!".to_owned()
        }
    }
}

pub struct MyRequestHandler;

impl RequestHandler<Request, Response> for MyRequestHandler {

    type Error = std::io::Error;

    fn handle(&self, req: &Request) -> Result<Response, Self::Error> {
        Ok(Response {
            welcome: format!("Hello, {}!", req.name)
        })
    }

}

