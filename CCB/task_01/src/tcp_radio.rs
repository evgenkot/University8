use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    match stream.peer_addr() {
        Ok(peer_addr) => println!("Got connection from {}", peer_addr),
        Err(err) => eprintln!("Failed to get peer address: {}", err),
    }

    if let Err(err) = stream.write_all(b"Hello from super server!\n") {
        eprintln!("Error writing to stream: {}", err);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "0.0.0.0:992".to_string();
    let listener_address = args.get(1).unwrap_or(&binding);

    let listener = TcpListener::bind(&listener_address).expect("Failed to bind to address");

    println!(
        "Server listening on {}",
        listener
            .local_addr()
            .expect("Failed to get listener addres")
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
