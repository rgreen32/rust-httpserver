use std::{collections::HashMap, io::Write, net::TcpListener, time::Duration};
use http_server_starter_rust::{request_handler, HttpRequest, HttpResponse, RequestLine};
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

                match request_handler::read_stream_into_request(&mut reader) {
                    Ok(request) => {
                        let response = request_handler::handle_request(request);
                        let response_string = response.to_string();
                        let response_bytes = response_string.as_bytes();
                        let _ = _stream.write_all(response_bytes);

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
                        let _ = _stream.write_all(response_bytes);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
