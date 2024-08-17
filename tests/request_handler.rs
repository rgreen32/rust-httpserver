use std::{collections::HashMap, net::TcpStream};
use std::io::Write;
use std::str::from_utf8;
use http_server_starter_rust::{request_handler, HttpRequest, HttpResponse, RequestLine};
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
fn test_deserialize_requestline_returns_requestline() {
    let requestline_string = "GET /abc HTTP/1.1".to_string();

    let result = request_handler::deserialize_requestline(requestline_string);

    assert_eq!(result.method, "GET");
    assert_eq!(result.target, "/abc");
    assert_eq!(result.version, "HTTP/1.1");
}

#[test]
fn test_deserialize_headers_returns_headers() {
    let requestheaders_string = "Date: Wed, 15 Aug 2024 12:00:00 GMT\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: 138\r\nConnection: keep-alive".to_string();

    let result = request_handler::deserialize_headers(requestheaders_string);

    assert_eq!(result.get("Date").unwrap(), "Wed, 15 Aug 2024 12:00:00 GMT");
    assert_eq!(result.get("Content-Type").unwrap(), "text/html; charset=UTF-8");
    assert_eq!(result.get("Content-Length").unwrap(), "138");
    assert_eq!(result.get("Connection").unwrap(), "keep-alive");
}

#[test]
fn test_read_stream_into_request() {
    // Mocking a TCP stream with in-memory data using Cursor
    let request = b"GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";

    let mut mock_stream = Cursor::new(request.to_vec());

    let request: HttpRequest = request_handler::read_stream_into_request(&mut mock_stream).unwrap();

    assert_eq!(request.request_line.method, "GET");
    assert_eq!(request.request_line.target, "/echo/abc");
    assert_eq!(request.request_line.version, "HTTP/1.1");
}

#[test]
fn test_handle_request_root() {
    let request = HttpRequest {
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
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
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/unknown".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
                                    headers: HashMap::new(),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);

    assert_eq!(response.status_code, 404);
    assert_eq!(response.reason_phrase, "Not Found");
}

#[test]
fn test_echo_path_returns_body_1(){

    let request = HttpRequest {
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/echo/abc".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
                                    headers: HashMap::new(),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);
    
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("Content-Length"));
    assert_eq!(response.body, "abc")
}

#[test]
fn test_echo_path_returns_body_2(){
    let request = HttpRequest {
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/echo/zzzzz_ff".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
                                    headers: HashMap::new(),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);
    
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("Content-Length"));
    assert_eq!(response.headers.get("Content-Length"), Some(&"8".to_string()));
    assert_eq!(response.body, "zzzzz_ff")
}

#[test]
fn test_echo_path_returns_body_3(){
    let request = HttpRequest {
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/echo/grape".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
                                    headers: HashMap::new(),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);
    
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("Content-Length"));
    assert_eq!(response.headers.get("Content-Length"), Some(&"5".to_string()));
    assert_eq!(response.body, "grape")
}

#[test]
fn test_useragent_path_returns_body_1(){
    let request = HttpRequest {
                                    request_line: RequestLine {
                                                        method: "GET".to_string(),
                                                        target: "/user-agent".to_string(),
                                                        version: "HTTP/1.1".to_string(),
                                                    },
                                    headers: HashMap::from([("User-Agent".to_string(), "foobar".to_string())]),
                                    body: String::new()
                                };

    let response = request_handler::handle_request(request);
    
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("Content-Length"));
    assert_eq!(response.headers.get("Content-Length"), Some(&"6".to_string()));
    assert_eq!(response.body, "foobar")
}
