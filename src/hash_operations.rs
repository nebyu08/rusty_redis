use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::export_type::RespValue;

pub  fn handle_set(items: &[RespValue], db: &Arc<Mutex<HashMap<String, String>>>) -> RespValue {
    if items.len() != 3 {
        return RespValue::Error("Wrong number of arguments for set".into());
    }

    let key = match &items[1] {
        RespValue::BulkString(k) => String::from_utf8_lossy(k).into_owned(),
        _ => return RespValue::Error("key must be a bulk string".into()),
    };

    let value = match &items[2] {
        RespValue::BulkString(v) => String::from_utf8_lossy(v).into_owned(),
        _ => return RespValue::Error("value must be a bulk strin".into()),
    };

    db.lock().unwrap().insert(key, value);
    RespValue::SimpleString("OK".into())
}

pub fn handle_get(items: &[RespValue], db: &Arc<Mutex<HashMap<String, String>>>) -> RespValue {
    if items.len() != 2 {
        return RespValue::Error("wrong number of arguments".into());
    }
    let key = match &items[1] {
        RespValue::BulkString(k) => String::from_utf8_lossy(k).into_owned(),
        _ => return RespValue::Error("key must be strin".into()),
    };

    match db.lock().unwrap().get(&key) {
        Some(value) => RespValue::BulkString(value.clone().into_bytes()),
        None => RespValue::Null,
    }
}
