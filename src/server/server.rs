use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to bind");
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                return;
            }
            stream
                .write(&buffer[0..size])
                .expect("Failed to write to stream");
            true
        }
        Err(_) => {
            eprintln!("Failed to read from stream");
            false
        }
    } {}
}
