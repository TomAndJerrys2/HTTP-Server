use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// present data to client
fn handle_connection(mut stream: TcpStream) {
    let mut conn_buffer = [0; 1024]; // 1024 bytes
    stream.read(&mut conn_buffer).unwrap();

    let get_header = b"GET / HTTP/1.1\r\n";

    // Handling page routing for errors
    let (status, file_name) = if conn_buffer.starts_with(get_header) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "NoPage.html")
    };

    let contents: String = fs::read_to_string(file_name).unwrap();

    let response: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
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
        handle_connection(stream);
    }
}
