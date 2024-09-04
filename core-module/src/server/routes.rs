#[warn(unused_imports)]
use std::time::SystemTime;
// use chrono::prelude::*;
use std::fs;

pub fn handle_homepage() -> String {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./static/index.html").unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );
    response
}

pub fn handle_docs() -> String {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./static/docs.html").unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );
    response
}

pub fn handle_echo() -> String {
    let response_body = r#"{"message": "hello"}"#;
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}


pub fn handle_greet(request_target: &str) -> String {
    let name = request_target.split('/').last().unwrap_or("User");
    let response_body = format!(r#"{{"greeting": "Hello, {}!"}}"#, name);
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}

pub fn handle_compute() -> String {
    let response_body = r#"{"result": 13}"#;
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}

pub fn handle_status() -> String {
    let response_body = r#"{"status": "OK", "uptime": "24 hours", "version": "1.0.0"}"#;
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}

pub fn handle_time() -> String {
    let current_time = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = current_time.into();
    let response_body = format!(r#"{{"current_time": "{}"}}"#, datetime.to_rfc3339());
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    )
}

pub fn handle_api_request(request_target: &str) -> String {
    let response_body = &request_target[5..]; // Strip off "/api/" from the request target
    let content_length = response_body.len();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content_length,
        response_body
    )
}
