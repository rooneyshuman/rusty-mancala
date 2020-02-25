use client::*;

fn main() {
    println!("Running client");
    let host: String = "localhost".to_string();
    let port: u32 = 3333;

    Client::new(host, port).run_client();
}
