#[warn(unused_imports)]
use std::time::SystemTime;
use std::path::Path;
use std::fs;

pub fn handle_homepage() -> String {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./static/homepage.html").unwrap();
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


pub fn handle_404_error() -> String {
    let mut response = String::new();
    let status_line = "HTTP/1.1 404 Not Found\r\n";
    let content_type = "Content-Type: text/html\r\n";
    let content = fs::read_to_string(Path::new("./static/404.html")).unwrap_or_else(|_| {
        // Fallback in case the file can't be read
        "<html><body><h1>404 Not Found</h1><p>The page you are looking for does not exist.</p></body></html>".to_string()
    });
    let content_length = format!("Content-Length: {}\r\n", content.len());
    
    response.push_str(status_line);
    response.push_str(content_type);
    response.push_str(&content_length);
    response.push_str("\r\n"); // End of headers
    response.push_str(&content);

    response
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_handle_homepage() {
        // Setup: Create a temporary index.html file in a temporary directory
        let test_dir = "./static";
        let test_file_path = Path::new(test_dir).join("index.html");

        // Ensure the directory exists
        fs::create_dir_all(test_dir).unwrap();

        // Write some test content to index.html
        let mut file = File::create(&test_file_path).unwrap();
        writeln!(file, "<h1>Tester for Rusty</h1>").unwrap();

        // Expected values
        let expected_status_line = "HTTP/1.1 200 OK";
        let expected_contents = "<h1>Tester for Rusty</h1>\n";
        let expected_length = expected_contents.len();
        let expected_response = format!(
            "{expected_status_line}\r\nContent-Length: {expected_length}\r\nContent-Type: text/html\r\n\r\n{expected_contents}"
        );

        // Run the function
        let response = handle_homepage();

        // Assert that the function returns the expected response
        assert_eq!(response, expected_response);

        // Cleanup: Remove the temporary file
        fs::remove_file(&test_file_path).unwrap();
    }

    #[test]
    fn test_handle_docs() {
        // Setup: Create a temporary docs.html file in a temporary directory
        let test_dir = "./static";
        let test_file_path = Path::new(test_dir).join("docs.html");

        // Ensure the directory exists
        fs::create_dir_all(test_dir).unwrap();

        // Write some test content to docs.html
        let mut file = File::create(&test_file_path).unwrap();
        writeln!(file, "<h1>Documentation</h1>").unwrap();

        // Expected values
        let expected_status_line = "HTTP/1.1 200 OK";
        let expected_contents = "<h1>Documentation</h1>\n";
        let expected_length = expected_contents.len();
        let expected_response = format!(
            "{expected_status_line}\r\nContent-Length: {expected_length}\r\nContent-Type: text/html\r\n\r\n{expected_contents}"
        );

        // Run the function
        let response = handle_docs();

        // Assert that the function returns the expected response
        assert_eq!(response, expected_response);

        // Cleanup: Remove the temporary file
        fs::remove_file(&test_file_path).unwrap();
    }
    
    #[test]
    fn test_handle_echo() {
        // Expected values
        let response_body = r#"{"message": "hello"}"#;
        let expected_status_line = "HTTP/1.1 200 OK";
        let expected_content_type = "application/json";
        let expected_length = response_body.len();
        let expected_response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            expected_status_line,
            expected_content_type,
            expected_length,
            response_body
        );

        // Run the function
        let response = handle_echo();

        // Assert that the function returns the expected response
        assert_eq!(response, expected_response);
    }
}

