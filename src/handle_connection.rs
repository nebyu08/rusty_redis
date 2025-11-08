
use tokio::net::TcpStream;
use tokio::sync::mpsc:: Sender;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{hash_operations::{handle_get, handle_set}, serial_deserial::DecodeResult};
use crate::serial_deserial::{decode_resp_value, encode_resp_value};
use crate::export_type::RespValue;
use crate::db_ops::DBMessage;

pub async fn handle_client(
    mut socket: TcpStream,
    db_sender:Arc::<Sender<DBMessage>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut temp_buff = [0u8; 1024];

    loop {
        let n = socket.read(&mut temp_buff).await?;
        if n == 0 {
            println!("Client disconnected");
            break Ok(());
        }
        

        buffer.extend_from_slice(&temp_buff[..n]);

        // while let Some((req,used_bytes  )) = decode_resp_value(&buffer){
        loop{
            match decode_resp_value(&buffer){
                DecodeResult::Complete(req, used_bytes) => {
                    buffer.drain(0..used_bytes);
                    let response = match req {
                        RespValue::Array(items) if !items.is_empty() => match items[0] {
                    RespValue::BulkString(ref cmd_bytes) => {
                        match std::str::from_utf8(cmd_bytes)
                            .unwrap()
                            .to_uppercase()
                            .as_str()
                        {
                            "PING" => RespValue::SimpleString("PONG".into()),
                            "SET" => handle_set(&items, &db_sender).await,
                            "GET" => handle_get(&items, &db_sender).await,
                            _ => RespValue::Error("unknown command".into()),
                        }
                    }
                    _ => RespValue::Error("Invalid command".into()),
                },

                _ => RespValue::Error("command must be array".into()),
            };

            let resp_bytes = encode_resp_value(&response);
            socket.write_all(&resp_bytes).await?;
            },
            DecodeResult::Incomplete => break,
            DecodeResult::Error(e) =>{
                socket.write_all(format!("-Error decoding request: {}\r\n", e).as_bytes()).await?;
                buffer.clear();
                break;
            }
        }
    }
}
}
