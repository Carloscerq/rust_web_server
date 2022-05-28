use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn send_ok_response(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_error_response(mut stream: TcpStream, error_code: &str, error_message: &str) {
    let response = format!("HTTP/1.1 {} {}\r\n\r\n", error_code, error_message);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



pub fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    send_ok_response(stream.try_clone().unwrap());
}
