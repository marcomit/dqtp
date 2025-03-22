use std::{thread, time::Duration};

use client::Client;

mod client;

fn main() {
    let rnd = format!("client_{}", rand::random_range(0..10));
    let client = Client::connect("localhost:8080", rnd.as_str());

    if client.is_none() {
        println!("Errore connessione");
        return;
    }
    let mut client = client.unwrap();

    let _ = client.recv(|buf| {
        println!("Ricevuto {}", String::from_utf8_lossy(buf));
    });
    let mut i = 0;
    loop {
        let to_write = format!("Messaggio {}", i);
        if let Err(_) = client.send(to_write.as_bytes()) {
            println!("Errore scrittura");
            break;
        }
        thread::sleep(Duration::from_secs(1));
        println!("scritto {}", to_write);
        i += 1;
    }
}
