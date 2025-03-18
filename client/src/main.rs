use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};

use client::Client;

mod client;

fn main() {
    let mut client = Client::connect("localhost:8080");
    
    let _ = client.recv(|buf| {
        println!("Letto {}", String::from_utf8_lossy(buf));
    });
    loop {
        let to_write = String::from("Client");
        client.send(to_write.as_bytes());
        thread::sleep(Duration::from_secs(1));
        println!("scritto {}", to_write);
    }
}
