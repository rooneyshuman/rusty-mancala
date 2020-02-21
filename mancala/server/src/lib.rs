use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // use a 512 byte buffer
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                // Exit loop if no bytes received
                if size == 0 {
                    println!("Client terminated connection");
                    break;
                }
                let input = str::from_utf8(&buffer[0..size]).unwrap().trim_end();
                println!("Data received: {}", input);
                // Exit loop & terminate connection if user enters "quit"
                if input.eq_ignore_ascii_case("quit") {
                    println!("Client terminated connection");
                    stream.shutdown(Shutdown::Both).unwrap();
                    break;
                } else {
                    // TODO: update & send game state instead of echoing
                    stream.write_all(&buffer[0..size]).unwrap();
                    stream.flush().unwrap();
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                // Close read & write portions of connection
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

pub fn run_server(host: String, port: u32) {
    let connection = [host, ":".to_string(), port.to_string()].concat();
    let listener = TcpListener::bind(connection).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded, handle stream thread
                    handle_client(stream)
                });
            }
            Err(e) => {
                // connection failed, print error received
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
