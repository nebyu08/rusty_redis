use std::sync::Arc;
use tokio::net::TcpListener;
mod hash_operations;
mod handle_connection;
mod export_type;
mod serial_deserial;
mod db_ops;
use crate::db_ops::start_db_thread;
use crate::handle_connection::handle_client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6381").await?;
 
    let db_sender=start_db_thread("snapshot.rdb");
    let db_sender=Arc::new(db_sender);

    loop {
        let (socket, _) = listener.accept().await?;
        // let db_clone = Arc::clone(&db);
        let db_sender_clone=db_sender.clone();

        tokio::spawn(async move {
            handle_client(socket, db_sender_clone)
                .await
                .unwrap_or_else(|e| eprint!("client error: {}", e));
        });
    }
}



        

