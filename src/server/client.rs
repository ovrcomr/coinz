use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn start_client(address: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connected to the server");
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                stream
                    .write(input.as_bytes())
                    .expect("Failed to write to stream");
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        if size == 0 {
                            return;
                        }
                        println!("Received: {}", String::from_utf8_lossy(&buffer[0..size]));
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stream: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
}
