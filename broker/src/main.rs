use std::{net::*, thread::*};

use broker::Broker;
use client::Client;
// use client::*;
// use broker::*;

mod client;
mod broker;
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
    let mut broker = Broker::get();
    let mut client = Client::new(stream);
    client.handle_connection();
    broker.add_client(client);
}
