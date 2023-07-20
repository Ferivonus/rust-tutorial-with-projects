use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc::set_handler;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set the Ctrl+C signal handler
    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");
    let ip = "0.0.0.0"; //will changed by you.
    let ip_with_socket = format!("{}:8080", ip);
    let mut stream = TcpStream::connect(&ip_with_socket).expect("Failed to connect");
    println!("Connected to the server.");

    while running.load(Ordering::SeqCst) {
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Send the message to the server
        if let Err(err) = stream.write_all(input.trim().as_bytes()) {
            println!("Error sending message: {}", err);
            break;
        }
    }
}
/*
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    ctrlc = "3.4.0"
*/