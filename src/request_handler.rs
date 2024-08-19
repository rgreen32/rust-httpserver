
use std::{fmt::Display, fs::File, io::{self, BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, path::Path, time::Duration};
use config::AppConfig;
use itertools::Itertools;

use super::*;

pub fn handle_request(request: HttpRequest) -> HttpResponse {
    let path_components: Vec<&str> = request.request_line.target[1..].split("/").collect();
    
    
    let response_params: (Option<u32>, Option<String>, Option<String>, Option<String>) = (None, None, None, None);

    let (status_code, reason_phrase, headers, body): (Option<u32>, Option<String>, Option<HashMap<String, String>>, Option<String>) = match path_components[0] {
                                                                                                                                                                    "" => {
                                                                                                                                                                        let status_code = 200;
                                                                                                                                                                        let reason_phrase = String::from("OK");
                                                                                                                                                                        
                                                                                                                                                                        (Some(status_code), Some(reason_phrase), None, None)
                                                                                                                                                                    }, 
                                                                                                                                                                    "echo" => {
                                                                                                                                                                        let status_code = 200;
                                                                                                                                                                        let reason_phrase = String::from("OK");
                                                                                                                                                                        
                                                                                                                                                                        let body = match path_components.get(1) {
                                                                                                                                                                            Some(value) => value.to_string(),
                                                                                                                                                                            None => String::new()
                                                                                                                                                                        };

                                                                                                                                                                        let headers = HashMap::from([
                                                                                                                                                                            ("Content-Type".to_string(), "text/plain".to_string()),
                                                                                                                                                                            ("Content-Length".to_string(), body.len().to_string())
                                                                                                                                                                        ]);

                                                                                                                                                                        (Some(status_code), Some(reason_phrase), Some(headers), Some(body))
                                                                                                                                                                    }, 
                                                                                                                                                                    "user-agent" => {
                                                                                                                                                                        let status_code = 200;
                                                                                                                                                                        let reason_phrase = String::from("OK");

                                                                                                                                                                        let body = match request.headers.get("User-Agent") {
                                                                                                                                                                            Some(value) => value.to_string(),
                                                                                                                                                                            None => String::new()
                                                                                                                                                                        };
                                                                                                                                                                        let headers = HashMap::from([
                                                                                                                                                                            ("Content-Type".to_string(), "text/plain".to_string()),
                                                                                                                                                                            ("Content-Length".to_string(), body.len().to_string())
                                                                                                                                                                        ]);

                                                                                                                                                                        (Some(status_code), Some(reason_phrase), Some(headers), Some(body))
                                                                                                                                                                    },
                                                                                                                                                                    "files" => {
                                                                                                                                                                        let mut status_code = 200;
                                                                                                                                                                        let mut reason_phrase = String::from("OK");

                                                                                                                                                                        let file_contents: Option<String> = match path_components.get(1) {
                                                                                                                                                                            Some(file_name) => {
                                                                                                                                                                                let file_path_string = AppConfig::global().serve_directory.clone() + file_name;
                                                                                                                                                                                let file_path = Path::new(&file_path_string);

                                                                                                                                                                                let mut file_contents_string: Option<String> = match File::open(file_path) {
                                                                                                                                                                                    Ok(mut file) => {
                                                                                                                                                                                        let mut file_contents_string = String::new();
                                                                                                                                                                                        match file.read_to_string(&mut file_contents_string) {
                                                                                                                                                                                            Ok(_) => Some(file_contents_string),
                                                                                                                                                                                            Err(_) => {
                                                                                                                                                                                                status_code = 500;
                                                                                                                                                                                                reason_phrase = String::from(format!("Error reading contents of {}", file_name));
                                                                                                                                                                                                None
                                                                                                                                                                                            } 
                                                                                                                                                                                        }
                                                                                                                                                                                    },
                                                                                                                                                                                    Err(e) => {
                                                                                                                                                                                        match e.kind() {
                                                                                                                                                                                            io::ErrorKind::NotFound => {
                                                                                                                                                                                                status_code = 404;
                                                                                                                                                                                                reason_phrase = String::from("Not Found");

                                                                                                                                                                                            },
                                                                                                                                                                                            _ => {
                                                                                                                                                                                                status_code = 500;
                                                                                                                                                                                                reason_phrase = String::from(format!("Error opening file: {}", file_name));
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                        None
                                                                                                                                                                                    }
                                                                                                                                                                                };

                                                                                                                                                                                file_contents_string
                                                                                                                                                                            },
                                                                                                                                                                            None => {
                                                                                                                                                                                None
                                                                                                                                                                            }
                                                                                                                                                                        };

                                                                                                                                                                        let (headers, body): (Option<HashMap<String, String>>, Option<String>) = match file_contents {
                                                                                                                                                                            Some(file_contents) => {
                                                                                                                                                                                let headers = HashMap::from([
                                                                                                                                                                                    ("Content-Type".to_string(), "application/octet-stream".to_string()),
                                                                                                                                                                                    ("Content-Length".to_string(), file_contents.len().to_string())
                                                                                                                                                                                ]);
                                                                                                                                                                                (Some(headers), Some(file_contents))
                                                                                                                                                                            },
                                                                                                                                                                            None => {
                                                                                                                                                                                (None, None)
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        
                                                                                                                                                                        (Some(status_code), Some(reason_phrase), headers, body)
                                                                                                                                                                    },    
                                                                                                                                                                    _ => {
                                                                                                                                                                        let status_code = 404;
                                                                                                                                                                        let reason_phrase = String::from("Not Found");

                                                                                                                                                                        (Some(status_code), Some(reason_phrase), None, None)
                                                                                                                                                                    }
                                                                                                                                                                };
    let response: HttpResponse = match status_code {
        Some(status_code) =>  HttpResponse {
            version: request.request_line.version,
            status_code: status_code,
            reason_phrase: reason_phrase.unwrap_or(String::new()),
            headers: headers.unwrap_or(HashMap::new()),
            body: body.unwrap_or(String::new())
        },
        None =>  HttpResponse {
            version: request.request_line.version,
            status_code: status_code.unwrap_or(500),
            reason_phrase: reason_phrase.unwrap_or(String::from("Unable to process request")),
            headers: HashMap::new(),
            body: String::new()
        }
    };
    return response
}




pub fn read_stream_into_request<Stream: BufRead>(stream: &mut Stream) -> Result<HttpRequest, io::Error> {
    let mut request_line_bytes: Vec<u8> = Vec::new();
    let mut headers_bytes: Vec<u8> = Vec::new();
    let mut body_bytes: Vec<u8> = Vec::new();

    
    let mut buffer = [0; 1024];
    let mut value_position = 0;
    let mut value_offset = 0;
    let mut value_length = 0;

    //request line
    while value_position != 2 {
        let bytes_read = stream.read(&mut buffer);
        match bytes_read {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let mut buffer_index = 0; 
                while buffer_index < bytes_read {
                    match value_position {
                        0 => {
                            let mut byte = buffer[buffer_index];
                            loop {
                                request_line_bytes.push(byte);
                                buffer_index += 1;

                                if buffer_index == bytes_read { break; }
                                byte = buffer[buffer_index];

                                if byte == 13 && buffer[buffer_index+1] == 10 { break; }
                            }

                            buffer_index += 2;
                            value_position +=1;
                        },
                        1 => {
                            let mut byte = buffer[buffer_index];
                            loop {
                                headers_bytes.push(byte);
                                buffer_index += 1;

                                if buffer_index == bytes_read { break; }
                                byte = buffer[buffer_index];

                                if byte == 13 && buffer[buffer_index+1] == 10 && buffer[buffer_index+2] == 13 && buffer[buffer_index+3] == 10 { break; }
                            }
                            buffer_index += 4;
                            value_position +=1;
                        },
                        2 => {
                            let mut byte = buffer[buffer_index];
                            loop {
                                body_bytes.push(byte);
                                buffer_index += 1;

                                if buffer_index == bytes_read { break; }
                                byte = buffer[buffer_index];
                                
                                if byte == 0 { break; }
                            }
                            value_position += 1;
                        },
                        _ => {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "Too many CRLF!"))
                        }
                        
                    }
                }

            },
            
            Err(err) => {
                return Err(err);
            }
        }

    }

    let request_line_string: String = match String::from_utf8(request_line_bytes) {
        Ok(string) => string,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not parse request_line into UTF8 string"))
    };
        
    let headers_string: String = match String::from_utf8(headers_bytes) {
        Ok(string) => string,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not parse headers into UTF8 string"))
    };

    let body_string: String = match String::from_utf8(body_bytes) {
        Ok(string) => string,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not parse body into UTF8 string"))
    };
    
    let request_line = deserialize_requestline(request_line_string);
    let request_headers = deserialize_headers(headers_string);
    
    let request = HttpRequest { 
                                    request_line: request_line,
                                    headers: request_headers,
                                    body: body_string
                                };
    return  Ok(request);
}



pub fn deserialize_requestline(request_line_string: String) -> RequestLine {
    let request_line_components: Vec<String> = request_line_string.split(" ").map(|s| s.to_string()).collect();

    return RequestLine {
                method: request_line_components[0].clone(),
                target: request_line_components[1].clone(),
                version: request_line_components[2].clone()
            }   
}

pub fn deserialize_headers(request_headers_string: String) -> HashMap<String, String> {
    let mut header_map: HashMap<String, String> = HashMap::new();

    let header_strings: Vec<String> = request_headers_string.split("\r\n").map(|s| s.to_string()).collect();

    for header_string in header_strings {
        if let Some((header_key, header_value)) = header_string.split_once(":"){
            header_map.insert(header_key.trim().to_string(), header_value.trim().to_string());
        }
    }

    return header_map;
}


pub fn accept_request_stream(stream: &mut TcpStream) {
    println!("accepted new connection ");
    stream.set_read_timeout(Some(Duration::new(1, 0)));

    let mut reader = BufReader::new(&*stream);

    match request_handler::read_stream_into_request(&mut reader) {
        Ok(request) => {
            let response = request_handler::handle_request(request);
            let response_string = response.to_string();
            let response_bytes = response_string.as_bytes();
            let _ = stream.write_all(response_bytes);

        },
        Err(e) => {
            let error_response = HttpResponse { 
                                                    version: String::from("HTTP/1.1"),
                                                    status_code: 500,
                                                    reason_phrase: e.to_string(),
                                                    headers: HashMap::new(),
                                                    body: String::new()  
                                                };
            let response_string = error_response.to_string();
            let response_bytes = response_string.as_bytes();
            let _ = stream.write_all(response_bytes);
        }
    }
}
