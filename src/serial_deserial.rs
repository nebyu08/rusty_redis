use crate::export_type::RespValue;

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

        RespValue::Null => {
            buffer.extend_from_slice(b"$-1\r\n");
        }

        RespValue::Error(s) => {
            buffer.push(b'-');
            buffer.extend_from_slice(s.as_bytes());
            buffer.extend_from_slice(b"\r\n");
        }
        // _ => unimplemented!(),
    }

    buffer
}

pub enum DecodeResult {
    Complete(RespValue, usize),
    Incomplete,
    Error(String),  
}

pub fn decode_resp_value(bytes: &[u8]) -> DecodeResult {
    if bytes.is_empty() {
        return DecodeResult::Incomplete;
    }

    const MAX_BULK_STRING_SIZE: usize = 512 * 1024 * 1024; // 512 MB

    match bytes[0] {
        b'+' => {
        if let Some(end_index) = bytes[1..].iter().position(|&b| b == b'\r'){
            let s =match String::from_utf8(bytes[1..end_index+1].to_vec()) {
                Ok(v) => v,
                Err(_) => return DecodeResult::Error("Invalid UTF-8".into())
      
            }; 
            return DecodeResult::Complete(RespValue::SimpleString(s), end_index + 3);
        // Some((RespValue::SimpleString(s), end_index + 2))
        }else {
            return DecodeResult::Incomplete;
        }
        }

        b':' => {

            if let Some(end_index) = bytes[1..].iter().position(|&b| b == b'\r'){
                let i_str = match String::from_utf8(bytes[1..end_index+1].to_vec()) {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid UTF-8".into()),      
                };
                let i_itr = match i_str.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid integer".into()),      
                };
                return DecodeResult::Complete(RespValue::Integer(i_itr), end_index + 2);
            } else {
                return DecodeResult::Incomplete;
            }
            // let i_str = String::from_utf8(bytes[1..end_index].to_vec()).ok()?;
            // let i_itr = i_str.parse::<i64>().ok()?;
            // Some((RespValue::Integer(i_itr), end_index + 2))
        }

        b'$' => {
            if let Some(len_end_index) = bytes[1..].iter().position(|&b| b == b'\r') {
                let len_str = match String::from_utf8(bytes[1..len_end_index+1].to_vec()) {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid UTF-8".into()),
                };
                let len = match len_str.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid integer".into()),
                };
                if len > MAX_BULK_STRING_SIZE {
                    return DecodeResult::Error("Bulk string too large".into());
                }
                let data_start = len_end_index + 2;
                let data_end = data_start + len;
                if data_end > bytes.len() {
                    return DecodeResult::Incomplete;
                }
                let bulk_string = bytes[data_start..data_end].to_vec();
                return DecodeResult::Complete(RespValue::BulkString(bulk_string), data_end + 2);
            } else {
                return DecodeResult::Incomplete;
            }
        }

        b'*' => {
            if let Some(len_end_index) = bytes[1..].iter().position(|&b| b == b'\r') {
                let len_str = match String::from_utf8(bytes[1..len_end_index+1].to_vec()) {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid UTF-8".into()),
                };
                let len = match len_str.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid integer".into()),
                };
                let mut elements = Vec::with_capacity(len);
                let mut current_offset = len_end_index + 2;

                for _ in 0..len {
                    match decode_resp_value(&bytes[current_offset..]) {
                        DecodeResult::Complete(element, parsed_len) => {
                            elements.push(element);
                            current_offset += parsed_len;
                        }
                        DecodeResult::Incomplete => {
                            return DecodeResult::Incomplete;
                        }
                        DecodeResult::Error(e) => {
                            return DecodeResult::Error(e);
                        }
                    }
                }
                return DecodeResult::Complete(RespValue::Array(elements), current_offset);
            } else {
                return DecodeResult::Incomplete;
            }
        }

        b'-' => {
            if let Some(end_index) = bytes[1..].iter().position(|&b| b == b'\r') {
                let s = match String::from_utf8(bytes[1..end_index+1].to_vec()) {
                    Ok(v) => v,
                    Err(_) => return DecodeResult::Error("Invalid UTF-8".into()),
                };
                return DecodeResult::Complete(RespValue::Error(s), end_index + 2);
            } else {
                return DecodeResult::Incomplete;
            }
        }

        _ => return DecodeResult::Error("Invalid resp value".into()),
    }
}
