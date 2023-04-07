//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

use redis::util::get_request;
use redis::util::del_request;
use redis::util::set_request;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    println!("created stream");

    let msg = set_request("hello", b"world");
    let res = stream.write(&msg).await;
    println!("wrote to stream; success={:?}", res.is_ok());

    Ok(())
}
