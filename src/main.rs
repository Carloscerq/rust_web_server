use std::net::{TcpListener, TcpStream};
use std::env;

fn main() {
    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(e) => panic!("{}", e) 
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
