use std::{io::{Read, Write}, net::*, thread::*};
// use client::*;
// use broker::*;

mod client;
// mod broker;
// mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        if stream.is_err() { continue; }
        let mut stream = stream.unwrap();
        let _ = spawn(move || handle_client(&mut stream));
    }
}

fn handle_client(stream: &mut TcpStream) {
    println!("New client");
    let to_write = "Ciao".as_bytes();
    let _ = stream.write_all(to_write);
    println!("scrivi {:?}", to_write);

    let buf: &mut String = &mut String::new();
    let _ = stream.read_to_string(buf);
    println!("letto {}", buf);
    // let client = Client::new(stream);

    // let mut broker = Broker::get();

    // broker.add_client(client);
}
