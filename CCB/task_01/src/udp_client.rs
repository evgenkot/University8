use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4242").expect("Failed to bind to address");
    let mut buffer = [0; 1024];
    loop {
        let mut input_string = String::new();
        std::io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
        socket.send_to(input_string.as_bytes(), "127.0.0.1:992").expect("Failed to send response");
        let (size, source) = socket.recv_from(&mut buffer).expect("Failed to receive data");
        let request = String::from_utf8_lossy(&buffer[..size]);
        println!("Received responce: {} from {}", request, source);
    }
}