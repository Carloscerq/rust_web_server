use std::net::{TcpListener, TcpStream};
mod handle_request;
use std::env;

fn main() {
    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(e) => panic!("{}", e) 
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_request::handle_request(&mut stream);

        println!("Connection established!");
    }
}
