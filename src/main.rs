use std::net::{TcpListener};
mod handle_request;
mod file_handler;
use std::env;

fn main() {
    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_e) => "8080".to_string(),
    };

    let path = match env::var("FILES") {
        Ok(v) => v,
        Err(_e) =>  env::current_dir().unwrap().to_str().unwrap().to_string() + "/public",
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Listening on port {}", port);
    println!("Serving files from {}", path);
    let files = file_handler::list_all_files_in_directory(path);
    let responses = handle_request::create_response(&files);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_request::handle_request(&mut stream, &responses);
    }
}
