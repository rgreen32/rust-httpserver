// Uncomment this block to pass the first stage
use std::{fmt::Display, io::{Read, Write}, net::{TcpListener, TcpStream}, time::Duration};


#[derive(Debug)]
struct HttpRequest {
    method: String,
    request_target: String,
    version: String,
    headers: Vec<String>
}

struct HttpResponse {
    version: String,
    status_code: u32,
    reason_phrase: String,
    headers: Vec<(String, String)>
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.version);
        write!(f, "{} ", self.status_code);
        write!(f, "{} ", self.reason_phrase);
        write!(f, "\r\n");

        for (header, value) in self.headers.iter(){
            write!(f, "{}: {}\r\n", header, value);
        } 

        return write!(f, "\r\n");
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection ");
                _stream.set_read_timeout(Some(Duration::new(1, 0)));
                let request_line = get_request_line(&mut _stream);
                let mut request = HttpRequest{ method: request_line.0, request_target: request_line.1, version: request_line.2, headers: Vec::new() };
                let response = service_request(request);
                let response_string = response.to_string();
                let repsonse_bytes =  response_string.as_bytes();
                let _ = _stream.write_all(repsonse_bytes);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn get_request_line(stream: &mut TcpStream) -> (String, String, String) {
    let mut request_line = (String::new(), String::new(), String::new());


    let mut buffer = [0; 1024];
    let mut value_position = 1;
    let mut value_offset = 0;
    let mut value_length = 0;
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        for (index, byte) in buffer[..bytes_read].iter().enumerate() {
            if *byte == 32 || *byte == 13 { // if value is " " or \r
                let mut value = &buffer[value_offset..value_offset+value_length];
                match value_position {
                    1 => {
                        
                        let _ = value.read_to_string(&mut request_line.0);
                    },
                    2 => {
                        let _ = value.read_to_string(&mut request_line.1);
                    },
                    3 => {
                        let _ = value.read_to_string(&mut request_line.2);
                    },
                    _ => {
                        panic!("First sequence line too big?");
                    }
                }
                value_offset = value_offset+ value_length + 1;
                value_position += 1;
                value_length = 0;
            } else {
                value_length += 1;
            }

            if index != buffer.len() - 1 && [*byte, buffer[index+1]] == [b'\r', b'\n'] {
                break;
            }
        }
    }

    return  request_line;
}

fn service_request(request: HttpRequest) -> HttpResponse {
    
    let response: HttpResponse = match request.request_target.as_str() {
        "/" => HttpResponse { version: request.version, status_code: 200, reason_phrase: String::from("OK"), headers: vec![(String::from("Date"), String::from("Wed, 15 Aug 2024 12:00:00 GMT"))] },
        _ => HttpResponse { version: request.version, status_code: 404, reason_phrase: String::from("Not Found"), headers: vec![(String::from("Date"), String::from("Wed, 15 Aug 2024 12:00:00 GMT"))] }
    };

    return response
}