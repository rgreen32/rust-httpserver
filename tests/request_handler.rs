use std::{collections::HashMap, net::TcpStream};
use std::io::Write;
use std::str::from_utf8;
use http_server_starter_rust::{request_handler, HttpRequest, HttpResponse};
use std::io::{Cursor, Read};


#[test]
fn test_get_request_line() {
    // Mocking a TCP stream with in-memory data using Cursor
    let request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let mut mock_stream = Cursor::new(request.to_vec());

    // Call the function
    let (method, request_target, version) = request_handler::get_request_line(&mut mock_stream).unwrap();

    // Assertions
    assert_eq!(method, "GET");
    assert_eq!(request_target, "/");
    assert_eq!(version, "HTTP/1.1");
}

#[test]
fn test_read_stream_into_request() {
    // Mocking a TCP stream with in-memory data using Cursor
    let request = b"HTTP/1.1 200 OK\r\nDate: Wed, 15 Aug 2024 12:00:00 GMT\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: 138\r\nConnection: keep-alive\r\n\r\n<html>\r\n<head><title>Sample Page</title></head>\r\n<body><h1>Hello, World!</h1></body>\r\n</html>\r\n";

    let mut mock_stream = Cursor::new(request.to_vec());

    let request: HttpRequest = request_handler::read_stream_into_request(&mut mock_stream).unwrap();

    assert_eq!(request.method, "GET");
    assert_eq!(request.request_target, "/");
    assert_eq!(request.version, "HTTP/1.1");
}

#[test]
fn test_handle_request_root() {
    let request = HttpRequest {
                                    method: "GET".to_string(),
                                    request_target: "/".to_string(),
                                    version: "HTTP/1.1".to_string(),
                                    headers: HashMap::new(),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);

    assert_eq!(response.status_code, 200);
    assert_eq!(response.reason_phrase, "OK");
}

#[test]
fn test_handle_request_404() {
    let request = HttpRequest {
                    method: "GET".to_string(),
                    request_target: "/unknown".to_string(),
                    version: "HTTP/1.1".to_string(),
                    headers: HashMap::new(),
                    body: String::new()
                };

    let response = request_handler::handle_request(request);

    assert_eq!(response.status_code, 404);
    assert_eq!(response.reason_phrase, "Not Found");
}

// #[test]
// fn test_echo_path_returns_body_1(){

//     let request = HttpRequest {
//                     method: "GET".to_string(),
//                     request_target: "/echo/abc".to_string(),
//                     version: "HTTP/1.1".to_string(),
//                     headers: HashMap::new(),
//                     body: String::new()
//                 };
//     let response = request_handler::handle_request(request);
    
//     assert_eq!(response.status_code, 200);
//     assert!(response.headers.contains_key("Content-Type"));
//     assert!(response.headers.contains_key("Content-Length"));
//     assert_eq!(response.body, "abc")
// }

// #[test]
// fn test_echo_path_returns_body_2(){
//     let request = HttpRequest {
//                     method: "GET".to_string(),
//                     request_target: "/echo/zzzzz_ff".to_string(),
//                     version: "HTTP/1.1".to_string(),
//                     headers: HashMap::new(),
//                     body: String::new()
//                 };
//     let response = request_handler::handle_request(request);
    
//     assert_eq!(response.status_code, 200);
//     assert!(response.headers.contains_key("Content-Type"));
//     assert!(response.headers.contains_key("Content-Length"));
//     assert_eq!(response.body, "abc")
// }