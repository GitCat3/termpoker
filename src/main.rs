use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666").await?;

    println!("Server is running on 127.0.0.1:6666");
    
    loop {
        // Accept a new incoming connection
        let (mut socket, _) = listener.accept().await?;

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // Read data from the client
            match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => {
                    if String::from_utf8_lossy(&buffer[..n]) == "connect" {
                        println!("connection!")
                    }
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                    // Echo the data back to the client
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        println!("Failed to send response: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to read from socket: {}", e);
                }
            }
        });
    }
}
