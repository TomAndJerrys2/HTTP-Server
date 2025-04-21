use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// present data to client
fn handleConnection(mut stream: TcpStream) {
    let mut connBuffer = [0; 1024]; // 1024 bytes

    stream.read(&mut connBuffer).unwrap();

    let contents: String = fs::read_to_string("index.html").unwrap();

    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handleConnection(stream);
    }
}
