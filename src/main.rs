use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

//Database setup. Ensure information up to date.
//SQLx, Tokio, NOAA API calls.
//Cache data to minimize requests, make API calls as needed.

//Server-side:
//Handle valid requests from various sources.
//Disregard invalid requests and raise errors.

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client.");
    //reads data sent from client, returns result.
    
    //convert to utf-8
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response.");
}

fn parse_request() {
    //Check for valid input. Support flags. Return some object to be sent to server.
}
//TODO: structure requests to REST standard. TYPE|BODY.
fn generate_response() {
    //Read object from client. Perform actions based on request type and flags.
}

//Entry point: Single-threaded server.
fn main() {
    let localhost = "127.0.0.1:8080";
    let listener = TcpListener::bind(localhost).
        expect("Failed to bind to address.");
    println!("Server listening on {}", localhost);

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
}
