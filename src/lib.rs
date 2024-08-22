use std::{collections::HashMap, fmt::Display};
use lazy_static::lazy_static;
pub mod request_handler;
pub mod config;

pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: String
}

pub struct RequestLine {
    pub method: String,
    pub target: String,
    pub version: String
}

pub struct HttpResponse {
    pub version: String,
    pub status_code: u32,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.version);
        write!(f, "{} ", self.status_code);
        write!(f, "{}", self.reason_phrase);
        write!(f, "\r\n");

        for (header, value) in self.headers.iter(){
            write!(f, "{}: {}\r\n", header, value);
        }
        write!(f, "\r\n");
    


        return write!(f, "{}", self.body); 
    }
}

