use std::{collections::HashMap, fs::File, io::{BufReader, BufWriter,Write}, path::PathBuf};
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


pub fn start_db_thread(path: impl Into<PathBuf>)->mpsc::Sender<DBMessage>{
    // configure the file path for the database
    let snapshot_path:PathBuf= path.into();

    let mut db= match File::open(&snapshot_path){
        Ok(file)=>serde_json::from_reader(BufReader::new(file)).unwrap_or_default(),
        Err(_)=>HashMap::new(),
    };

    let (tx,mut rx) =  mpsc::channel(1024);

    thread::spawn(async move || {
        // let mut db: HashMap<String,String>=HashMap::new();

        while let Some(message) = rx.recv().await {
            match message{
                DBMessage::Set{key,value}=>{
                    db.insert(key,value);
                    let db_clone=db.clone();
                    let path_clone=snapshot_path.clone();
                    tokio::task::spawn_blocking(move || persistance_db(&db_clone,&path_clone) );
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

fn persistance_db(db: &HashMap<String,String>,path: &PathBuf)->Result<(),String>{
    let tmp = path.with_extension("rdb.tmp");
    let file = File::create(&tmp).map_err(|e| format!("error creating a temporary file {} ",e))?;
    let mut writer= BufWriter::new(file);
    serde_json::to_writer(&mut writer, db).map_err(|e| format!("error writing to temporary file {}",e))?;
    writer.flush().map_err(|e| format!("error flushing temporary file {} ",e))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("error renaming temporary file {} ",e))?;
    Ok(())
}
