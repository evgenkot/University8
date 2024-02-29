use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn handle_response(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(string_bytes) => {
            let response = String::from_utf8_lossy(&buffer[..string_bytes]);
            println!("Server response: {}", response);
        }
        Err(err) => eprintln!("Failed to read buffer from server: {}", err),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "127.0.0.1:992".to_string();
    let server_address = args.get(1).unwrap_or(&binding);

    let mut stream = TcpStream::connect(server_address).unwrap();

    println!("Connected to server at {}", server_address);

    handle_response(&mut stream);
    loop {
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
        stream.write_all(input_string.trim().as_bytes()).unwrap();
        handle_response(&mut stream);
    }
}
