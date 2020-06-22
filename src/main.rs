use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from {}", stream.peer_addr()?);
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buffer[.. bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("cant bind socket");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| eprintln!("issue {:?}", err));
                });
            }
            Err(e) => {eprintln!("failed {}", e)}
        }
    }
}