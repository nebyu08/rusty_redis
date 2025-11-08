use std::sync::Arc;
// use std::sync::mpsc:: Sender;
use tokio::sync::mpsc:: Sender;
use tokio::sync::oneshot;

use crate::db_ops::DBMessage;
use crate::export_type::RespValue;

pub async fn handle_set(items: &[RespValue], db_sender: &Arc::<Sender<DBMessage>>) -> RespValue {
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

    println!("set key: {}, value: {}", key, value);

    db_sender.send(DBMessage::Set { key, value }).await.unwrap();

    // db.lock().unwrap().insert(key, value);
    RespValue::SimpleString("OK".into())
}

pub async  fn handle_get(items: &[RespValue],db_sender: &Arc::<Sender<DBMessage>>) -> RespValue {
    if items.len() != 2 {
        return RespValue::Error("wrong number of arguments".into());
    }
    let key = match &items[1] {
        RespValue::BulkString(k) => String::from_utf8_lossy(k).into_owned(),
        _ => return RespValue::Error("key must be strin".into()),
    };

    let (response_tx, response_rx) =oneshot::channel();

    db_sender.send(DBMessage::Get { key, response_sender: response_tx }).await.expect("failed to get command");

    match response_rx.await{
        Ok(Some(val))=>RespValue::BulkString(val.into_bytes()),
        Ok(None) => RespValue::Null,
        Err(_) => RespValue::Error("DB thread crashed".into())
    }

}
