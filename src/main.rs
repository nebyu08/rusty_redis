use std::io::{self, Write};
use std::sync::Arc;
use tokio::net::TcpListener;
mod db_ops;
mod export_type;
mod handle_connection;
mod hash_operations;
mod serial_deserial;

use crate::db_ops::start_db_thread;
use crate::handle_connection::handle_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("supppppppp");
    io::stdout().flush().unwrap();

    match TcpListener::bind("127.0.0.1:6381").await {
        Ok(listener) => {
            eprintln!("Server started at 127.0.0.1:6381");

            let db_sender = start_db_thread("snapshot.rdb");
            let db_sender = Arc::new(db_sender);

            loop {
                let (socket, _) = listener.accept().await?;
                // let db_clone = Arc::clone(&db);
                let db_sender_clone = db_sender.clone();

                tokio::spawn(async move {
                    handle_client(socket, db_sender_clone)
                        .await
                        .unwrap_or_else(|e| eprint!("client error: {}", e));
                });
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    // Ok(())
}
