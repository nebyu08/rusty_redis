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
            buffer.extend_from_slice(b"\n\r");
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
            // Some((RespValue::SimpleString((s)), end_index + 2))
            // Some(RespValue::SimpleString((s)), end_index + 2)
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
