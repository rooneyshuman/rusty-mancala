use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

#[derive(Debug, Default, Clone)]
pub struct Client {
    connection: String,
}

impl Client {
    pub fn new(host: String, port: u32) -> Client {
        Client {
            connection: [host, ":".to_string(), port.to_string()].concat(),
        }
    }

    pub fn run_client(self) {
        match TcpStream::connect(self.connection) {
            Ok(mut stream) => loop {
                let mut input = String::new();
                let mut buffer: Vec<u8> = Vec::new();
                // Read input from user
                io::stdin().read_line(&mut input).expect("I/O error");
                // Exit loop & terminate connection if user enters "quit"
                if input.trim_end().eq_ignore_ascii_case("quit") {
                    println!("Goodbye!");
                    break;
                }
                // Write user input to server
                stream
                    .write_all(input.as_bytes())
                    .expect("Server write error");
                stream.flush().unwrap();
                // Create read stream & read input from server
                let mut reader = BufReader::new(&stream);
                reader.read_until(b'\n', &mut buffer).expect("Buffer error");
                print!("{}", str::from_utf8(&buffer).expect("Buffer->String error"));
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Connection terminated");
    }
}
