use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    TcpListener::bind("0.0.0.0:8080")?.incoming().for_each(|connection| {
        match connection {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                let _ = &stream.read(&mut buf).expect("Error reading from stream");
                let request = String::from_utf8_lossy(&buf).trim_matches(char::from(0)).to_string();
                let body = request.split("\r\n\r\n").collect::<Vec<&str>>()[1];
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", body);
                let _ = stream.write(&response.as_bytes()).expect("Error writing to stream");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    });

    Ok(())
}
