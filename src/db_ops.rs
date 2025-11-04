use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::{thread};

// operations to be done on the db
pub enum DBMessage {
    Set {
        key:String,
        value:String
    },

    Get {
        key:String,
        response_sender:Sender<Option<String>>,
    },
    // Remove {
    //     key:String,
    // },
}

pub fn start_db_thread()->Sender<DBMessage>{
    let (tx,rx) =  mpsc::channel::<DBMessage>();

    thread::spawn(move || {
        let mut db: HashMap<String,String>=HashMap::new();

        for messag in rx{
            match messag{
                DBMessage::Set{key,value}=>{
                    db.insert(key,value);
                },

                DBMessage::Get{key,response_sender}=>{
                    let value=db.get(&key).cloned();
                    response_sender.send(value).expect("Failed to get send response");
                }

                // DBMessage::Remove { key }=>{
                //     db.remove(&key);
                // }
            }
        }
    });

    tx

}