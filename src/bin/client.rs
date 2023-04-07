//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use std::env;
use std::error::Error;

use redis::util::get_request;
use redis::util::del_request;
use redis::util::set_request;
use redis::util::from_response;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    println!("created stream");

    let req = parse_request();
    let res = stream.write(&req).await;
    println!("wrote to stream; success={:?}", res.is_ok());

    println!("reading response from stream");
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.expect("failed to read from stream");
    if n == 0 {
        return Ok(());
    }
    println!("response: {}", from_response(&buf[0..n].to_vec()));

    Ok(())
}

fn parse_request() -> Vec<u8> {
    let default = vec![1];
    if env::args().len() == 3 {
        return match env::args().nth(1).unwrap().as_str() {
            "get" => get_request(&env::args().nth(2).unwrap()),
            "del" => del_request(&env::args().nth(2).unwrap()),
            _ =>  default,
        };
    } else if env::args().len() == 4 {
        return match env::args().nth(1).unwrap().as_str() {
            "set" => set_request(
                env::args().nth(2).unwrap().as_str(),
                env::args().nth(3).unwrap().as_bytes()),
            _ => default,
        };
    }
    default
}