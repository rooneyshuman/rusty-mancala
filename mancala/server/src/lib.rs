use client::*;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

#[derive(Debug, Default, Clone)]
pub struct Server {
    client_list: Vec<Client>,
    game_list: Vec<String>, // TODO: make it a vector of game states - placeholder
    connection: String,
}

impl Server {
    pub fn new(host: String, port: u32) -> Server {
        Self {
            client_list: vec![],
            game_list: vec![],
            connection: [host, ":".to_string(), port.to_string()].concat(),
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512]; // use a 512 byte buffer
        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    // Exit loop if no bytes received (client connection ended)
                    if size == 0 {
                        println!("Client terminated connection");
                        break;
                    }
                    let input = str::from_utf8(&buffer[0..size]).unwrap().trim_end();
                    println!("Data received: {}", input);

                    // TODO: update & send game state instead of echoing
                    stream.write_all(&buffer[0..size]).unwrap();
                    stream.flush().unwrap();
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

    pub fn run_server(&self) {
        let listener = TcpListener::bind(self.connection).unwrap();
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        // connection succeeded, handle stream thread
                        self.handle_client(stream)
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
}
