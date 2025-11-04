
use tokio::net::TcpStream;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::hash_operations::{handle_get, handle_set};
use crate::serial_deserial::{decode_resp_value, encode_resp_value};
use crate::export_type::RespValue;

pub async fn handle_client(
    mut socket: TcpStream,
    db: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // handle stream cases
    let mut buffer = Vec::new();
    let mut temp_buff = [0u8; 1024];

    loop {
        let n = socket.read(&mut temp_buff).await?;
        if n == 0 {
            println!("Client disconnected");
            break Ok(());
        }

        buffer.extend_from_slice(&temp_buff[..n]);

        while let Some((req,used_bytes  )) = decode_resp_value(&buffer){
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
                            "SET" => handle_set(&items, &db),
                            "GET" => handle_get(&items, &db),
                            _ => RespValue::Error("unknown command".into()),
                        }
                    }
                    _ => RespValue::Error("Invalid command".into()),
                },

                _ => RespValue::Error("command must be array".into()),
            };

            let resp_bytes = encode_resp_value(&response);
            socket.write_all(&resp_bytes).await?;
        }
    }
}