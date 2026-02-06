use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use sqlx::sqlite::SqlitePoolOptions;

//Initialize connection to database.
//Listen for client connection attempts.
//Close on process exit.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //startup:hosted on local port
    //startup:database connection

    //client-server: connection
    //client-server: receive request
    //client-server: issue response

    //exit: close connection

    Ok(())
}
