use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// mod handle_connection;
mod hash_operations;
mod handle_connection;
mod export_type;
mod serial_deserial;

use crate::handle_connection::handle_client;

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



        

