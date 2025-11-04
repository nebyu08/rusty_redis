use std::sync::Arc;
use std::sync::mpsc:: Sender;

use crate::db_ops::DBMessage;
use crate::export_type::RespValue;

pub  fn handle_set(items: &[RespValue], db_sender: &Arc::<Sender<DBMessage>>) -> RespValue {
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

    db_sender.send(DBMessage::Set { key, value }).expect("failed to send to db");

    // db.lock().unwrap().insert(key, value);
    RespValue::SimpleString("OK".into())
}

pub fn handle_get(items: &[RespValue],db_sender: &Arc::<Sender<DBMessage>>) -> RespValue {
    if items.len() != 2 {
        return RespValue::Error("wrong number of arguments".into());
    }
    let key = match &items[1] {
        RespValue::BulkString(k) => String::from_utf8_lossy(k).into_owned(),
        _ => return RespValue::Error("key must be strin".into()),
    };

    let (response_tx, response_rx) = std::sync::mpsc::channel();

    db_sender.send(DBMessage::Get { key, response_sender: response_tx }).expect("failed to get command");

    match response_rx.recv(){
        Ok(Some(val))=>RespValue::BulkString(val.into_bytes()),
        Ok(None) => RespValue::Null,
        Err(_) => RespValue::Error("DB thread crashed".into())
    }

}
