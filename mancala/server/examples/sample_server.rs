use server::*;

fn main() {
    println!("Running server");
    let host: String = "localhost".to_string();
    let port: u32 = 3333;

    run_server(host, port);
}
