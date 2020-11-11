use super::server::Handler;
use super::http::{Request, Response, StatusCode};

#[derive(Debug)]
pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>"))
    }
}