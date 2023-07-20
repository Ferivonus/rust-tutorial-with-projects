use std::io::Read;
use std::net::TcpListener;
use tokio::runtime::Runtime;
use std::error::Error;
use std::thread; // Add this line to import the `thread` module
    // Attempt to get an IP address and return it.

async fn ip_finder() -> Result<String, Box<dyn Error>> {
    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        Ok(ip.to_string())
    } else {
        Err("Failed to parse public IP address".into())
    }
}

fn handle_client(mut stream: std::net::TcpStream) {
    let addr = stream.peer_addr().expect("Failed to get peer address");
    println!("Accepted connection from: {}", addr);

    // Read incoming messages
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed by peer");
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received message: {}", message);
            }
            Err(err) => {
                println!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let ip = runtime.block_on(ip_finder()).unwrap();
    let address = format!("{}:8080", ip);
    let listener = TcpListener::bind(&address).expect("Failed to bind");
    println!("Listening for incoming connections...");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(move || {
                    handle_client(stream);
                });
                handle.join().unwrap();
            }
            Err(err) => {
                println!("Error accepting incoming connection: {}", err);
            }
        }
    }
}

/*
    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    tokio = "1.29.1"
    public-ip = "0.2.2"
    thread = "0.1.0"
*/

