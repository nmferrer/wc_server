use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use ws_api;

//Database setup. Ensure information up to date.
//SQLx, Tokio, NOAA API calls.
//Cache data to minimize requests, make API calls as needed.

//Server-side:
//Handle valid requests from various sources.
//Disregard invalid requests and raise errors.

//Some considerations for handling data:
//How should requests look like?
//How should responses look like?
//How do I handle faulty data?
//What data sources am I pulling from?
//How do I most effectively use database and queries?
//What best practices should I keep in mind?

//Tasks
//CONFIGURE DB (running on server, accessible by main code)
//CONFIGURE TCP/IP (server can receive connections from clients, send data back and forth)

//DATABASE: Connection | Asynchronous Query Execution | Error Handling
fn db_setup() {

}
//DATABASE: Connection | Asynchronous Query Execution | Error Handling


fn handle_client(mut stream: TcpStream) {
    //Linting suggestion: Handle partial read/write
    let mut buffer = [0; 1024];
    stream.read_exact(&mut buffer).expect("Failed to read from client.");
    //reads data sent from client, returns result.
    //convert to utf-8
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, client!".as_bytes();
    stream.write_all(response).expect("Failed to write response.");
}

fn parse_request() {
    //Check for valid input. Support flags. Return some object to be sent to server.
}
//TODO: structure requests to REST standard. TYPE|BODY.
fn generate_response() {
    //Read object from client. Perform actions based on request type and flags.
}

//CONSIDER MOVING CONFIGURATION TO LIBRARY?

//Entry point: Single-threaded server.

use sqlx::mysql::MySqlPoolOptions;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let localhost = "127.0.0.1:8080";
    let listener = TcpListener::bind(localhost).
        expect("Failed to bind to address.");
    println!("Server listening on {}", localhost);

    //Server connects to db. No direct connections from clients.
    //API handles client-server connections and passing data.
    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect("mysql://mysql:password@localhost/test:").await?;
    
    //test query
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool).await?;
    assert_eq!(row.0, 150);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }

    Ok(())
}
