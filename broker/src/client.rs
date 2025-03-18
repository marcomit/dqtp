use std::{collections::HashSet, io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use serde_json::Value;

// use crate::broker::{Broker, BROKER};

pub struct Client {
  stream: Arc<Mutex<TcpStream>>,
  subscription: HashSet<Vec<String>>
}

impl Client {
  pub fn new(stream: &mut TcpStream) -> Self {
    let stream = Arc::new(Mutex::new(stream.try_clone().unwrap()));
    println!("New client");
    Self { stream, subscription: HashSet::new() }
  }

  pub fn send(&mut self, buf: &[u8]) {
    let stream = Arc::clone(&self.stream);
    let mut stream = stream.lock().unwrap();
    stream.write(buf).unwrap();
    stream.flush().unwrap();
  }

  pub fn recv<F>(&mut self, mut callback: F) -> JoinHandle<()>
  where F: FnMut(&[u8]) + Send + 'static, {
      let stream = Arc::clone(&self.stream);
      thread::spawn(move || {
          let mut stream = stream.lock().unwrap();
          let mut buf = vec![0; 4096];  // Start with 4KB, but we can resize if needed
          loop {
              match stream.read(&mut buf) {
                  Ok(0) => break,
                  Ok(n) => {
                      if n == buf.len() {
                          // Buffer was filled, might need more space
                          buf.resize(buf.len() * 2, 0);
                      }
                      callback(&buf[..n]);
                  },
                  Err(e) => {
                      println!("Errore {}", e);
                  }
              }
          }
      })
  }

  pub fn handle_connection(&mut self) {
    self.send("Ciao".as_bytes());

    self.recv(move |buf| {
      println!("Letto {}", String::from_utf8_lossy(buf));
      
    });
  }

  fn parse_command(&mut self, buf: &[u8]) {

  }

  pub fn subscribe(&mut self, path: Vec<String>) {
    self.subscription.insert(path);
  }

  pub fn unsubscribe(&mut self, path: Vec<String>) {

  }
  pub fn is_subscribed(&self, path: Vec<String>) -> bool {
    self.subscription.contains(&path)
  }
}