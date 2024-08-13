// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener, time::Duration};

use nom::AsChar;

#[derive(Debug)]
struct HttpRequestHeader {
    method: String,
    request_target: String,
    version: String,
    headers: Vec<String>
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


                let mut request_header = HttpRequestHeader{ method: String::new(), request_target: String::new(), version: String::new(), headers: Vec::new() };
                let mut buffer = [0; 1024];
                let mut value_position = 1;
                let mut value_offset = 0;
                let mut value_length = 0;
                while let Ok(bytes_read) = _stream.read(&mut buffer) {
                    if bytes_read == 0 {
                        break;
                    }

                    for (index, byte) in buffer[..bytes_read].iter().enumerate() {
                        if *byte == 32 || *byte == 13{
                            let mut value = &buffer[value_offset..value_offset+value_length];
                            match value_position {
                                1 => {
                                    let _ = value.read_to_string(&mut request_header.method);
                                },
                                2 => {
                                    let _ = value.read_to_string(&mut request_header.request_target);
                                },
                                3 => {
                                    let _ = value.read_to_string(&mut request_header.version);
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

                println!("Request Header: {:?}", request_header);


                 let response: &str = match request_header.request_target.as_str() {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n"
                };
                
                let repsonse_bytes =  response.as_bytes();
                let _ = _stream.write_all(repsonse_bytes);


            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
