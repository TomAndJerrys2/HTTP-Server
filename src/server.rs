// Main HTTP Server start

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

    // Handling the client request to the server
    priv async fn handle_req() {}
    
    // Handling the server response
    priv async fn handle_res() {}

    // Configure the HTTP Servers File System
    priv async fn config_router() {}

    // present data to client
    priv async fn handle_connection(mut stream: TcpStream) {
        let mut conn_buffer = [0; 1024]; // 1024 bytes
        stream.read(&mut conn_buffer).unwrap();

        let get_header = b"GET / HTTP/1.1\r\n";
        let sleep_header = b"GET /sleep HTTP/1.1\r\n";

        // Handling page routing for errors
        // config_router() here
        let (status, file_name) = if conn_buffer.starts_with(get_header) {
            ("HTTP/1.1 200 OK", "index.html");
        } else if conn_buffer.starts_with(sleep_header) {
            thread::sleep(Duration::from_secs(5));
        } else {
            ("HTTP/1.1 404 NOT FOUND", "NoPage.html");
        };

        // handle_req() here

        let contents: String = fs::read_to_string(file_name).unwrap();

        let response: String = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            contents.len(),
            contents
        );

        // handle_res() here

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    // Start a HTTP Server
    pub fn start_server(&mut self, mut server_addr: String, mut server_port: String) {
        self.address = server_addr;
        self.port = server_port;

        let listener = TcpListener::bind("{server_addr}:{server_port}")
            .expect("[-] Failed to bind to Server Host");

        let thread_pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            thread_pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
}