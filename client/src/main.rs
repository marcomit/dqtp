use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};

mod client;

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();

    let to_write = String::from("Client");
    let _ = stream.write_all(to_write.as_bytes());
    println!("scritto {}", to_write);
}
