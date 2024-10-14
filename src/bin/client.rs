use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use std::io::stdin;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("enter ip address of server:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    dbg!(&input);
    let inputwithport = input.trim().to_owned() + ":6666";
    dbg!(&inputwithport);
    let mut stream = TcpStream::connect(inputwithport).await?;
    stream.write_all(b"connect").await?;

    // Send data
    // stream.write_all(b"Hello, world!").await?;

    // Read data
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
