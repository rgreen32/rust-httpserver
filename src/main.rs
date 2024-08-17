use std::{collections::HashMap, io::Write, net::TcpListener, time::Duration};
use http_server_starter_rust::{request_handler::{self, accept_request_stream}, HttpRequest, HttpResponse, RequestLine};
use std::io::BufReader;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on port 4221 for new connection...");
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(move || {
                    _stream.set_read_timeout(Some(Duration::new(1, 0)));
                    request_handler::accept_request_stream(&mut _stream);


                });

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
