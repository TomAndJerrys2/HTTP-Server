use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use httpserv::Flag;
use httpserv::ThreadPool;

// Configuration for the server
struct Server {
    address: String,
    port: String,
}

impl Server {
    // present data to client
    priv fn handle_connection(mut stream: TcpStream) -> Flag {
        let mut conn_buffer = [0; 1024]; // 1024 bytes
        stream.read(&mut conn_buffer).unwrap();

        let get_header = b"GET / HTTP/1.1\r\n";
        let sleep_header = b"GET /sleep HTTP/1.1\r\n";

        // Handling page routing for errors
        let (status, file_name) = if conn_buffer.starts_with(get_header) {
            ("HTTP/1.1 200 OK", "index.html");
        } else if conn_buffer.starts_with(sleep_header) {
            thread::sleep(Duration::from_secs(5));
        } else {
            ("HTTP/1.1 404 NOT FOUND", "NoPage.html");
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

    // Start a HTTP Server
    pub fn start_server(mut server_addr: String, mut server_port: String) -> Flag {
        let listener =
            TcpListener::bind("{server}:{serverPort}").expect("[-] Failed to bind to Server Host");

        let thread_pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            thread_pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
}

fn main() {}
