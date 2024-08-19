use std::{env, net::TcpListener, time::Duration};
use http_server_starter_rust::{request_handler};
use std::thread;
use once_cell::sync::OnceCell;
use http_server_starter_rust::{config::{AppConfig, APP_CONFIG_INSTANCE}};


fn main() {

    //Create AppConfig instance which will hold configuration values
    let app_config = AppConfig::initialize();

    //claim socket and listen for TCP connections
    let listener = TcpListener::bind(format!("127.0.0.1:{}", app_config.port)).unwrap();
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
