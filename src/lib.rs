use std::fmt::Display;

pub mod request_handler;
pub struct HttpRequest {
    pub method: String,
    pub request_target: String,
    pub version: String,
    pub headers: Vec<String>
}

pub struct HttpResponse {
    pub version: String,
    pub status_code: u32,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>
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