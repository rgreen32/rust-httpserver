use std::{collections::HashMap, fmt::Display};

pub mod request_handler;

pub struct HttpRequest {
    pub method: String,
    pub request_target: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String
}


pub struct HttpRequest2 {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: String
}

pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub version: String
}
impl From<String> for RequestLine {
    fn from(request_line: String) -> Self {
        return RequestLine {
            method: String::new(),
            request_target: String::new(),
            version: String::new()
        }
    }
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

        return write!(f, "\r\n");
    }
}