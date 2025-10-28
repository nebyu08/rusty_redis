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

    let _ = server.kill().await;
}

#[tokio::test]
async fn test_ping() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .spawn()
        .expect("Failed to start server");

    sleep(Duration::from_millis(300)).await;

    let mut stream = TcpStream::connect("127.0.0.1:6381").await.unwrap();
    stream.write_all(b"PING\r\n").await.unwrap();

    let mut buf = vec![0u8; 7]; // "+PONG\r\n" = 7 bytes
    stream.read_exact(&mut buf).await.unwrap();

    assert_eq!(&buf, b"+PONG\r\n");
    let _ = server.kill().await;
}

#[tokio::test]
async fn test_set() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .spawn()
        .expect("Failed to start server");

    sleep(Duration::from_millis(300)).await;

    let mut stream = TcpStream::connect("127.0.0.1:6381").await.unwrap();
    stream.write_all(b"SET name nebiyu\r\n").await.unwrap();

    let mut buf = vec![0u8; 5]; // "+OK\r\n" = 5 bytes
    stream.read_exact(&mut buf).await.unwrap();

    assert_eq!(&buf, b"+OK\r\n");
    let _ = server.kill().await;
}

#[tokio::test]
async fn test_get() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .spawn()
        .expect("Failed to start server");

    sleep(Duration::from_millis(300)).await;

    let mut stream = TcpStream::connect("127.0.0.1:6381").await.unwrap();

    stream.write_all(b"SET lang rust\r\n").await.unwrap();
    let mut tmp = vec![0u8; 5];
    stream.read_exact(&mut tmp).await.unwrap(); // "+OK\r\n"

    stream.write_all(b"GET lang\r\n").await.unwrap();

    let mut buf = vec![0u8; 10]; // e.g. "$4\r\nrust\r\n"
    let size = stream.read(&mut buf).await.unwrap();

    let reply = String::from_utf8_lossy(&buf[..size]);
    assert!(reply.contains("rust"));

    let _ = server.kill().await;
}
