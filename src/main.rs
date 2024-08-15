

use std::{io::{Write}, net::{TcpListener}, time::Duration};
use http_server_starter_rust::{request_handler, HttpRequest, HttpResponse};
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on port 4221 for new connection...");
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection ");
                _stream.set_read_timeout(Some(Duration::new(1, 0)));
                let mut reader = BufReader::new(&_stream);
                let request_line = request_handler::get_request_line(&mut reader);
                let mut request = HttpRequest { 
                                                    method: request_line.0,  
                                                    request_target: request_line.1, 
                                                    version: request_line.2,
                                                    headers: Vec::new()
                                               };
                let response = request_handler::handle_request(request);
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
