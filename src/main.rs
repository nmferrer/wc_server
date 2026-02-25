use std::io;
use tokio::net::{TcpListener, TcpStream};
use sqlx::sqlite::SqlitePoolOptions;

//Initialize connection to database.
//Listen for client connection attempts.
//Close on process exit.

async fn process_socket<T>(socket: T) {
    println!("DEBUG: PROCESS SOCKET");
    //client-server: receive request
    //client-server: issue response
}

#[tokio::main]
async fn main() -> io::Result<()> {
    //startup:database connection
    let opts = SqliteConnectOptions::from_str("sqlite://data.db")?
        .journal_mode(SqliteJournalMode::Wal); //Note performance considerations between modes?
        //NOT read-only, though only server has access.
    let conn = opts.connect().await?; //no connection pool. Only server accesses db.
    //TODO: Connect to pool, not single connection
    //TODO: Pass connection to db_api, read/write through api

    //startup:hosted on local port | Connection OK, handle input from client
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?; //error handling?
        process_socket(socket).await;
    } //client-server: connection
    

    //exit: close connection

    Ok(())
}
