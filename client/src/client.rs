use std::{
    collections::HashSet,
    io::{Error, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use serde_json::Value;

pub struct Client {
    identifier: String,
    stream: Arc<Mutex<TcpStream>>,
    subscriptions: HashSet<Vec<String>>,
}

impl Client {
    pub fn connect(addr: &str, id: &str) -> Option<Self> {
        let stream = TcpStream::connect(addr);

        if let Err(e) = stream {
            return None;
        }
        let stream = Arc::new(Mutex::new(stream.unwrap()));
        let subscriptions = HashSet::new();
        let identifier = id.to_string();

        let mut client = Self {
            identifier,
            stream,
            subscriptions,
        };

        if let Err(_) = client.send(&format!("{}\n", id).as_bytes()) {
            return None;
        }

        Some(client)
    }
    pub fn send(&mut self, buf: &[u8]) -> Result<(), Error> {
        let stream = Arc::clone(&self.stream);
        let mut stream = stream.lock().unwrap();
        if let Err(e) = stream.write(buf) {
            return Err(e);
        }
        if let Err(e) = stream.flush() {
            return Err(e);
        }
        Ok(())
    }

    pub fn recv<F>(&self, callback: F) -> JoinHandle<()>
    where
        F: FnMut(&[u8]) + Send + 'static,
    {
        let stream_clone = Arc::clone(&self.stream);
        let callback = Arc::new(Mutex::new(callback));

        thread::spawn(move || {
            let mut stream = stream_clone.lock().unwrap();
            let mut buffer: &mut [u8] = &mut [];

            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = &buffer[..n];
                        let mut cb = callback.lock().unwrap();
                        println!("Ricevuto");
                        cb(data);
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                        break;
                    }
                }
            }
        })
    }

    // pub fn subscribe(&mut self, path: Vec<String>) {
    //     self.subscriptions.insert(path);
    // }

    // pub fn publish(&mut self, _path: Vec<String>, _value: Value) {}
}
