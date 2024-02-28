use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket}, thread::sleep, time::Duration,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "127.0.0.1:992".to_string();
    let listener_address = args.get(1).unwrap_or(&binding);

    let mut employees = HashMap::new();
    employees.insert("john".to_string(), "manager".to_string());
    employees.insert("jane".to_string(), "steno".to_string());
    employees.insert("jim".to_string(), "clerk".to_string());
    employees.insert("jack".to_string(), "salesman".to_string());

    let socket = UdpSocket::bind(&listener_address).expect("Failed to bind to address");

    println!(
        "Server listening on {}",
        socket
            .local_addr()
            .expect("Failed to get listener address")
    );
    let mut buffer = [0; 1024];
    loop {
        let (size, source) = socket.recv_from(&mut buffer).expect("Failed to receive data");
        let request = String::from_utf8_lossy(&buffer[..size]).trim().to_lowercase();

        let job = employees.get(&request).map_or_else(|| "No such employee", |job| job.as_str());

        socket.send_to(job.as_bytes(), source.to_string()).expect("Couldn't send data");
    }
    
}

