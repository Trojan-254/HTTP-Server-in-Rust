use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use log::info;
use std::error::Error;

mod routes;
use routes::{
    handle_homepage,
    handle_api_request,
    handle_compute,
    handle_echo,
    handle_greet,
    handle_time,
    handle_status,
    handle_docs,
};


pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    info!("Received request: {:#?}", http_request);
    

    if let Some(request_line) = http_request.first() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() == 3 {
            let method = parts[0];
            let request_target = parts[1];

            let response = match (method, request_target) {
                ("GET", "/") => handle_homepage(),
                ("GET", "/api/docs") => handle_docs(),
                ("GET", "/api/custom/{str}") => handle_api_request(request_target),
                ("GET","/api/echo") => handle_echo(),
                ("GET", "/api/greet") => handle_greet(request_target),
                ("POST", "/api/compute") => handle_compute(),
                ("GET", "/api/status") => handle_status(),
                ("GET", "/api/time") => handle_time(),
                (_, _) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            };

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    Ok(())
}

