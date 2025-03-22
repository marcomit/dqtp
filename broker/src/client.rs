use std::{
    collections::HashSet,
    io::{Error, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::broker::Broker;

// use crate::broker::{Broker, BROKER};

pub struct Client {
    identifier: Option<String>,
    stream: Arc<Mutex<TcpStream>>,
    subscription: HashSet<Vec<String>>,
}

impl Client {
    pub fn new(stream: &mut TcpStream) -> Self {
        let stream = Arc::new(Mutex::new(stream.try_clone().unwrap()));
        println!("New client");
        Self {
            identifier: None,
            stream,
            subscription: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> Option<String> {
        self.identifier.clone()
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

    pub fn recv<F>(&mut self, mut callback: F) -> JoinHandle<()>
    where
        F: FnMut(&[u8]) + Send + 'static,
    {
        let stream = Arc::clone(&self.stream);
        thread::spawn(move || {
            let mut stream = stream.lock().unwrap();
            let mut buf = vec![0; 4096];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if n == buf.len() {
                            buf.resize(buf.len() * 2, 0);
                        }
                        callback(&buf[..n]);
                    }
                    Err(e) => {
                        println!("Errore {}", e);
                    }
                }
            }
        })
    }

    pub fn handle_connection(&mut self) {
        self.send("Ciao".as_bytes()).unwrap();

        let identifier = Arc::new(Mutex::new(self.identifier.clone()));
        let identifier_clone = Arc::clone(&identifier);

        self.recv(move |buf| {
            let mut id = identifier_clone.lock().unwrap();
            if id.is_none() {
                *id = Some(String::from_utf8_lossy(buf).to_string());
                return;
            }
            let id = id.clone().unwrap();
            let mut broker = Broker::get();
            broker.handle(id, &buf);
        });
    }

    // fn parse_command(&mut self, buf: &[u8]) {}

    // pub fn subscribe(&mut self, path: Vec<String>) {
    //     self.subscription.insert(path);
    // }

    // pub fn unsubscribe(&mut self, path: &Vec<String>) {
    //     if !self.is_subscribed(path.clone()) {
    //         ()
    //     }
    //     self.subscription.remove(path);
    // }
    // pub fn is_subscribed(&self, path: Vec<String>) -> bool {
    //     self.subscription.contains(&path)
    // }
}
