extern crate string_repr;
extern crate wdg_uri;

pub mod header;
pub mod method;

use header::Header;
use method::Method;
//use std::error::Error;
//use string_repr::StringRepr;
use wdg_uri::URI;
//
//pub enum RequestError {
//    URIDoesNotContainAuthority,
//}
//
pub struct Request<'a> {
    method: Method,
    uri: URI<'a>,
    headers: Vec<Header<'a>>,
    data: Option<Vec<u8>>,
}

//impl Request {
//    pub fn new(method: Method, uri: URI) -> Result<Request, RequestError> {
//        let mut headers = Vec::<header>::new();
////        match &uri.authority {
////            Some(authority) => {
////                let header = header::new("Host".into(), authority.string_repr());
////                headers.push(header);
////            }
////            None => return Err(RequestError::URIDoesNotContainAuthority),
////        }
//        Ok(Request {
//            method,
//            uri,
//            headers,
//            data: None,
//        })
//    }
//    pub fn add_header(&mut self, header: header) {
//        self.headers.push(header);
//    }
//    pub fn set_data(&mut self, data: Vec<u8>) {
//        self.data = Some(data)
//    }
//    pub fn build(&self) -> String {
//        let mut string = String::new();
//        match &self.method {
//            Method::GET => string.push_str("GET "),
//            Method::POST => string.push_str("POST "),
//            Method::HEAD => string.push_str("HEAD "),
//        }
//        string
//    }
//}
