use std::{
  collections::HashSet,
  io::{Read, Write},
  net::TcpStream,
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
};

use serde_json::Value;

pub struct Client {
  stream: Arc<Mutex<TcpStream>>,
  subscriptions: HashSet<Vec<String>>,
}

impl Client {
  pub fn connect(addr: &str) -> Self {
      let stream = TcpStream::connect(addr).unwrap();
      let stream = Arc::new(Mutex::new(stream));
      let subscriptions = HashSet::new();

      Self { stream, subscriptions }
  }
  pub fn send(&mut self, buf: &[u8]) {
      let stream = Arc::clone(&self.stream);
      let mut stream = stream.lock().unwrap();
      stream.write(buf);
      stream.flush();
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

  pub fn subscribe(&mut self, path: Vec<String>) {
      self.subscriptions.insert(path);
  }

  pub fn publish(&mut self, _path: Vec<String>, _value: Value) {
  }
}