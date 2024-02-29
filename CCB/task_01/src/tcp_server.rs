use std::{
    collections::HashMap,
    env,
    io::{Write, Read},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream, employees: &HashMap<String, String>) {
    match stream.peer_addr() {
        Ok(peer_addr) => println!("Got connection from {}", peer_addr),
        Err(err) => eprintln!("Failed to get peer address: {}", err),
    }

    if let Err(err) = stream.write_all(b"Hello from super server!\nWrite name to get job title") {
        eprintln!("Error writing to stream: {}", err);
        return;
    }

    let mut buffer = [0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            println!("Client disconnected");
            return;
        }

        let input = String::from_utf8_lossy(&buffer[..n]).trim().to_lowercase();
        let job = employees.get(&input).map_or_else(|| "No such employee", |job| job.as_str());

        if let Err(err) = stream.write_all(job.as_bytes()) {
            eprintln!("Error writing to stream: {}", err);
            return;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "0.0.0.0:992".to_string();
    let listener_address = args.get(1).unwrap_or(&binding);

    let mut employees = HashMap::new();
    employees.insert("john".to_string(), "manager".to_string());
    employees.insert("jane".to_string(), "steno".to_string());
    employees.insert("jim".to_string(), "clerk".to_string());
    employees.insert("jack".to_string(), "salesman".to_string());

    let listener = TcpListener::bind(&listener_address).expect("Failed to bind to address");

    println!(
        "Server listening on {}",
        listener
            .local_addr()
            .expect("Failed to get listener address")
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let employees_clone = employees.clone();
                std::thread::spawn(move || {
                    handle_client(stream, &employees_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
