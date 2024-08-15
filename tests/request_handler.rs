use std::net::TcpStream;
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
    let (method, request_target, version) = request_handler::get_request_line(&mut mock_stream);

    // Assertions
    assert_eq!(method, "GET");
    assert_eq!(request_target, "/");
    assert_eq!(version, "HTTP/1.1");
}


#[test]
fn test_service_request_root() {
    let request = HttpRequest {
                                    method: "GET".to_string(),
                                    request_target: "/".to_string(),
                                    version: "HTTP/1.1".to_string(),
                                    headers: vec![],
                                };

    let response = request_handler::handle_request(request);

    assert_eq!(response.status_code, 200);
    assert_eq!(response.reason_phrase, "OK");
}

#[test]
fn test_service_request_404() {
    let request = HttpRequest {
                                    method: "GET".to_string(),
                                    request_target: "/unknown".to_string(),
                                    version: "HTTP/1.1".to_string(),
                                    headers: vec![],
                                };

    let response = request_handler::handle_request(request);

    assert_eq!(response.status_code, 404);
    assert_eq!(response.reason_phrase, "Not Found");
}
