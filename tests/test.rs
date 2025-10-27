use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    process::Command,
    time::sleep,
};

#[tokio::test]
async fn test_echo_server() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .spawn()
        .expect("Failed to start a server");

    sleep(Duration::from_millis(300)).await;
    let mut stream = TcpStream::connect("127.0.0.1:6381")
        .await
        .expect("Failed to connect to the server");

    let message = b"hello, echo";
    stream.write_all(message).await.unwrap();

    let mut buf = vec![0u8; message.len()];
    stream.read_exact(&mut buf).await.unwrap();

    assert_eq!(&buf, message);

    let _ = server.kill();
}

fn resp() {}
