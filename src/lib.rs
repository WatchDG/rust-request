extern crate string_repr;
extern crate wdg_uri;

pub mod header;
pub mod method;

use header::Header;
use method::Method;
use string_repr::StringRepr;
use wdg_uri::URI;

pub struct Request {
    method: Method,
    uri: URI,
    headers: Vec<Header>,
    data: Option<Vec<u8>>,
}

impl Request {
    pub fn new(method: Method, uri: URI) -> Request {
        let header = Header::new("Host".into(), uri.authority.string_repr());
        let mut headers = Vec::<Header>::new();
        headers.push(header);
        Request {
            method,
            uri,
            headers,
            data: None,
        }
    }
    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data)
    }
}
