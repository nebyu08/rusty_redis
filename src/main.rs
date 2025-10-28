use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[derive(Debug, PartialEq)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>),
    Null,
    Boolean(bool),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6381").await?;
    // request => deserialize => catagorizes it into number and string
    let db = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        let db_clone = Arc::clone(&db);

        tokio::spawn(async move {
            handle_client(socket, db_clone)
                .await
                .unwrap_or_else(|e| eprint!("client error: {}", e));
        });
    }
}

async fn handle_client(
    mut socket: TcpStream,
    db: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket.read(&mut buf).await?;
        //deserialize here
        if n == 0 {
            return Ok(());
        }
        let slice = &buf[..n];
        let (req, _) = match decode_resp_value(slice) {
            Some(v) => v,
            None => {
                socket.write_all(b"-Error invalid RESP\r\n").await?;
                continue;
            }
        };

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
                _ => RespValue::Error(("Invalid command".into())),
            },

            _ => RespValue::Error("command must be array".into()),
        };
    }
}

fn handle_set(items: &[RespValue], db: &Arc<Mutex<HashMap<String, String>>>) -> RespValue {
    if items.len() != 3 {
        return RespValue::Error("Wrong number of arguments for set".into());
    }

    let key= match &items[1]{
        RespValue::BulkString(k)=> String::from_utf8_lossy(k).into_owned(),
    }
}

fn handle_get() {}

pub fn encode_resp_value(value: &RespValue) -> Vec<u8> {
    let mut buffer = Vec::new();
    match value {
        RespValue::SimpleString(s) => {
            buffer.push(b'+');
            buffer.extend_from_slice(s.as_bytes());
            buffer.extend_from_slice(b"\r\n");
        }

        RespValue::Integer(i) => {
            buffer.push(b':');
            buffer.extend_from_slice(i.to_string().as_bytes());
            buffer.extend_from_slice(b"\r\n");
        }

        RespValue::BulkString(bytes) => {
            buffer.push(b'$');
            buffer.extend_from_slice(bytes.len().to_string().as_bytes());
            buffer.extend_from_slice(b"\r\n");
            buffer.extend_from_slice(bytes);
            buffer.extend_from_slice(b"\r\n");
        }

        RespValue::Array(elements) => {
            buffer.push(b'*');
            buffer.extend_from_slice(elements.len().to_string().as_bytes());
            buffer.extend_from_slice(b"\r\n");
            for element in elements {
                buffer.extend_from_slice(&encode_resp_value(element));
            }
        }
        _ => unimplemented!(),
    }

    buffer
}

pub fn decode_resp_value(bytes: &[u8]) -> Option<(RespValue, usize)> {
    if bytes.is_empty() {
        return None;
    }

    match bytes[0] {
        b'+' => {
            let end_index = bytes[1..].iter().position(|&b| b == b'\r')? + 1;
            let s = String::from_utf8(bytes[1..end_index].to_vec()).ok()?;
            Some((RespValue::SimpleString(s), end_index + 2))
        }

        b':' => {
            let end_index = bytes[1..].iter().position(|&b| b == b'\r')? + 1;
            let i_str = String::from_utf8(bytes[1..end_index].to_vec()).ok()?;
            let i_itr = i_str.parse::<i64>().ok()?;
            Some((RespValue::Integer(i_itr), end_index + 2))
        }

        b'$' => {
            let len_end_index = bytes[1..].iter().position(|&b| b == b'\r')? + 1;
            let len_str = String::from_utf8(bytes[1..len_end_index].to_vec()).ok()?;
            let len = len_str.parse::<usize>().ok()?;
            // let data_start = len_end_index + 3;
            let data_start = len_end_index + 2;
            let data_end = data_start + len;
            let bulk_string = bytes[data_start..data_end].to_vec();
            Some((RespValue::BulkString(bulk_string), data_end + 2))
        }

        b'*' => {
            let len_end_index = bytes[1..].iter().position(|&b| b == b'\r')? + 1;
            let len_str = String::from_utf8(bytes[1..len_end_index].to_vec()).ok()?;
            let len = len_str.parse::<usize>().ok()?;
            let mut elements = Vec::with_capacity(len);
            let mut current_offset = len_end_index + 2;

            for _ in 0..len {
                if let Some((element, parsed_len)) = decode_resp_value(&bytes[current_offset..]) {
                    elements.push(element);
                    current_offset += parsed_len;
                } else {
                    return None;
                }
            }
            Some((RespValue::Array(elements), current_offset))
            // Some((RespValue::Array(elements)),current_offset)
        }

        _ => None,
    }
}
