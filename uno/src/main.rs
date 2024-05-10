use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PrimeRequest {
    method: String,
    number: i32,
}

#[derive(Serialize)]
struct PrimeResponse {
    method: String,
    prime: bool,
}

impl From<PrimeRequest> for PrimeResponse {
    fn from(request: PrimeRequest) -> Self {
        PrimeResponse {
            prime: (2..request.number).all(|a| request.number % a != 0),
            method: request.method
        }
    }
}

fn read_request(stream: &mut TcpStream) -> Result<PrimeRequest> {
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;

    let body = String::from_utf8_lossy(&buf)
        .trim_matches(char::from(0))
        .to_string();
    
    // keep last line
    let body = body.lines().last().unwrap();

    let request = serde_json::from_str(&body)?;
    Ok(request)
}

fn handle_connection(stream: &mut TcpStream) -> Result<usize> {
    match read_request(stream).map(PrimeResponse::from) {
        Ok(response) => success(response, stream),
        Err(_) => failure(stream),
    }
}

fn success (response: PrimeResponse, stream: &mut TcpStream) -> Result<usize> {
    let response = serde_json::to_string(&response)?;
    // println!("Response: {:?}", response);
    stream.write(response.as_bytes()).with_context(|| "Cannot write to stream".to_string())
}

fn failure (stream: &mut TcpStream) -> Result<usize> {
    // println!("Nope!");
    stream.write("nope!".as_bytes()).with_context(|| "Cannot write to stream".to_string())
}

fn main() -> Result<()> {
    // println!("Server started");
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match handle_connection(&mut stream?) {
            Ok(_) => println!("Connection successful"),
            Err(e) => println!("Connection failed: {:?}", e),
        }
    }
    Ok(())
}
