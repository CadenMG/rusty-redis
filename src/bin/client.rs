//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    println!("created stream");

    let result1 = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result1.is_ok());

    Ok(())
}
