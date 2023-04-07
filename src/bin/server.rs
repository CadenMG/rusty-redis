use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use bytes::Bytes;

use redis::cmd::Command;
use redis::db::DB;
use redis::util::{to_response, SUCCESS_AND_DATA, SUCCESS_AND_NO_DATA, FAILED};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:1234 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1234".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let db = DB::new();

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;
        // Grab handle to the shared DB
        let handle = db.clone();

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let maybe_cmd = Command::parse_bytes(&buf, n);
                match maybe_cmd {
                    Ok(cmd) => {
                        let res = cmd.apply(&handle);
                        let status = if res.is_some() { 
                            SUCCESS_AND_DATA
                        } else { 
                            SUCCESS_AND_NO_DATA
                        };
                        println!("writing: {:?}", res);
                        socket
                            .write_all(&to_response(status, res))
                            .await
                            .expect("failed to write to client");
                    },
                    Err(e) => {
                        let res = format!("unknown message recieved: {}", e);
                        println!("{}", res);
                        let bytes = Some(Bytes::from(res));
                        socket
                            .write_all(&to_response(FAILED, bytes))
                            .await
                            .expect("failed to write to client");
                    }
                };
            }
        });
    }
}
