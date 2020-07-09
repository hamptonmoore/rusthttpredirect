use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8 ;4096];
    let req;
    let path;

    match stream.read(&mut buf) {
        Ok(_) => {
            req = String::from_utf8_lossy(&buf);
            path = req.split(" ").nth(1).unwrap();
            },
        Err(e) => {
            println!("Unable to read stream: {}", e);
            path = "/";
        },
    }

    let response = format!("{}{}{}", "HTTP/1.1 301 Moved Permanently\r\nLocation: https://hamptonmoore.com", path ,"\r\n\r\n");
    
    stream.write(response.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8084").unwrap();
    println!("Listening for connections on port {}", 8084);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}