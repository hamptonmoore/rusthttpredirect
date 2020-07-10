use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8084").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let req;
            let path;

            // In a loop, read data from the socket and write the data back.
            match socket.read(&mut buf).await {
                // socket closed
                Ok(_) => {
                    req = String::from_utf8_lossy(&buf);
                    path = req.split(" ").nth(1).unwrap();
                },
                Err(e) => {
                    path = "/";
                    eprintln!("failed to read from socket; err = {:?}", e);                    
                }
            };

            let mut message = b"HTTP/1.1 301 Moved Permanently\r\nContent-Length: 0\r\nLocation: https://hamptonmoore.com".to_vec();
            message.append(&mut path.as_bytes().to_vec());
            message.append(&mut b"\r\n\r\n".to_vec());
            if let Err(e) = socket.write_all(&message).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }

        });
    }
}