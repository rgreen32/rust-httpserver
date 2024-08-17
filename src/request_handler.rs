
use std::{fmt::Display, io::{self, BufRead, Read, Write}, net::{TcpListener, TcpStream}, time::Duration};
use itertools::Itertools;

use super::*;


pub fn handle_request(request: HttpRequest) -> HttpResponse {
    let path_components: Vec<&str> = request.request_line.target[1..].split("/").collect();
    
    let response: HttpResponse = match path_components[0] {
        "" => HttpResponse { 
                    version: request.request_line.version,
                    status_code: 200,
                    reason_phrase: String::from("OK"),
                    headers: HashMap::new(),
                    body: String::new()
                }, 
        "echo" => HttpResponse { 
                        version: request.request_line.version,
                        status_code: 200,
                        reason_phrase: String::from("OK"),
                        headers: HashMap::from([
                                        ("Content-Type".to_string(), "text/plain".to_string()),
                                        ("Content-Length".to_string(), "3".to_string())
                                    ]),
                        body: path_components[1].to_string()
                    }, 
        _ => HttpResponse {
                    version: request.request_line.version,
                    status_code: 404,
                    reason_phrase: String::from("Not Found"),
                    headers: HashMap::new(),
                    body: String::new()
                }
    };

    return response
}


pub fn get_request_line<Stream: BufRead>(stream: &mut Stream) -> Result<(String, String, String), io::Error> {
    let mut request_line = (String::new(), String::new(), String::new());


    let mut buffer = [0; 1024];
    let mut value_position = 0;
    let mut value_offset = 0;
    let mut value_length = 0;
    while value_position != 3 {
        let bytes_read = stream.read(&mut buffer);
        match bytes_read {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                for (index, byte) in buffer[..bytes_read].iter().enumerate() {
                    if *byte == 32 || *byte == 13 { // if value is " " or \r
                        let mut value = &buffer[value_offset..value_offset+value_length];
                        match value_position {
                            0 => {
                                
                                let _ = value.read_to_string(&mut request_line.0);
                            },
                            1 => {
                                let _ = value.read_to_string(&mut request_line.1);
                            },
                            2 => {
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
            },
            
            Err(err) => {
                return Err(err);
            }
        }

    }

    return  Ok(request_line);
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
    while value_position != 3 {
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