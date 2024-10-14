use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Send data
    stream.write_all(b"Hello, world!").await?;

    // Read data
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
