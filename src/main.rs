// use std::error::Error;

// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6381").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => {
                    if socket.write_all(&buf[0..n]).await.is_err() {
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket {}", e);
                    return;
                }
            };
        });
    }
}

#[tokio::test]
async fn test_echo_server() {
    use tokio::{
        // io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    };
    let mut stream = TcpStream::connect("127.0.0.1:6381").await.unwrap();

    let msg = b"hello world";
    stream.write_all(msg).await.unwrap();

    let mut buf = vec![0; msg.len()];
    stream.read_exact(&mut buf).await.unwrap();

    assert_eq!(&buf, msg);
}
