use std::io::prelude::*;
use std::net::{TcpStream};

fn send_error_response(stream: &mut TcpStream, error_code: &str, error_message: &str) {
    let content = std::fs::read_to_string(format!("src/error_pages/{}.html", error_code)).unwrap();
    let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", error_code, error_message, content.len(), content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Request: GET /contato HTTP/1.1
pub fn create_response(files: &Vec<String>) -> Vec<(String, String)> {
    let mut headers = Vec::new();
    for file in files {
        let contents = std::fs::read_to_string(file).unwrap();
        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        let header: String;
        let path = file.split("/").last().unwrap().split(".").collect::<Vec<&str>>();

        if path[0] == "index" {
            header = "GET / HTTP/1.1".to_string();
        } else {
            header = format!("GET /{} HTTP/1.1", path[0]);
        }

        headers.push((response, header));
    }
    headers
}

fn send_response(stream: &mut TcpStream, response: &Vec<(String, String)>, request: &str) {
    for (response, header) in response {
        if request.contains(header) {
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            break;
        }
    }

    send_error_response(stream, "404", "Not Found");
}

pub fn handle_request(stream: &mut TcpStream, responses: &Vec<(String, String)>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    send_response(stream, responses, &request);
}
