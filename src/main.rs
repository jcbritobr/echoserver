use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

// This functions is used to consum and handle a incomming socked
//
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from {}", stream.peer_addr()?);

    // create a buffer to hold the data from message
    let mut buffer = [0; 512];
    loop {
        // read the data into buffer(its why is mutable reference)
        let bytes_read = stream.read(&mut buffer)?;
        
        //if bytes_read is zero, there is nothing more to read, then return Ok
        if bytes_read == 0 {
            return Ok(());
        }

        // write data in socket(we want an echo server)
        stream.write(&buffer[.. bytes_read])?;
    }
}

fn main() {
    // creates a server socket bint at any interface of machine its running, in port 8080
    let listener = TcpListener::bind("0.0.0.0:8080").expect("cant bind socket");

    // iterate for ever incomming socket that connects to server
    for stream in listener.incoming() {
        match stream {
            // if there are a socket, pass it to handle_client function and spawn a thread for the socket
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| eprintln!("issue {:?}", err));
                });
            }
            // in case of error, just print the issue in stderr
            Err(e) => {eprintln!("failed {}", e)}
        }
    }
}