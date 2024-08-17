
use std::{fmt::Display, io::{self, BufRead, Read, Write}, net::{TcpListener, TcpStream}, time::Duration};
use super::*;


pub fn handle_request(request: HttpRequest) -> HttpResponse {
    
    
    let response: HttpResponse = match request.request_target.as_str() {
        "/" => HttpResponse { 
                    version: request.version,
                    status_code: 200,
                    reason_phrase: String::from("OK"),
                    headers: HashMap::new(),
                    body: String::new()
                }, 
        _ => HttpResponse {
                    version: request.version,
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
    let mut request_line_string = String::new();
    let mut headers_string = String::new();
    let mut body_string = String::new();

    
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

                for (index, byte) in buffer[..bytes_read].iter().enumerate() {
                    if *byte == 13 && buffer[index+1] == 10  { // if value is \r\n
                        let mut value = &buffer[value_offset..value_offset+value_length];
                        match value_position {
                            0 => {
                                let _ = value.read_to_string(&mut request_line_string);
                            },
                            1 => {
                                let _ = value.read_to_string(&mut headers_string);
                            },
                            2 => {
                                let _ = value.read_to_string(&mut body_string);
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
                }
            },
            
            Err(err) => {
                return Err(err);
            }
        }

    }

    println!("request_line_string: {:?}", request_line_string);
    
    let request = HttpRequest { 
                                    method: request_line_string,
                                    request_target: String::new(),
                                    version: String::new(),
                                    headers: HashMap::new(),
                                    body: String::new()
                                };
    return  Ok(request);
}

