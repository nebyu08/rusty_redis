use std::collections::HashMap;
// use std::sync::mpsc::{self, Sender};
use tokio::sync::{oneshot,mpsc};
use std::{thread};


#[derive(Debug)]
pub enum DBMessage {
    Set {
        key:String,
        value:String
    },

    Get {
        key:String,
        response_sender:oneshot::Sender<Option<String>>,
        // response_sender:Sender<Option<String>>,
    },
    // Remove {
    //     key:String,
    // },
}


pub fn start_db_thread()->mpsc::Sender<DBMessage>{
    let (tx,mut rx) =  mpsc::channel(1024);

    thread::spawn(async move || {
        let mut db: HashMap<String,String>=HashMap::new();

        while let Some(message) = rx.recv().await {
            match message{
                DBMessage::Set{key,value}=>{
                    db.insert(key,value);
                },

                DBMessage::Get{key,response_sender}=>{
                    let value=db.get(&key).cloned();
                    let _ =response_sender.send(value);
                    // response_sender.send(value).expect("Failed to get send response");
                }

                // DBMessage::Remove { key }=>{
                //     db.remove(&key);
                // }
            }
        }
    });

    tx

}