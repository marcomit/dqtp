use std::{collections::HashSet, iter::Map, net::TcpStream};

enum Value {
  Null,
  Bool(bool),
  Int(i32),
  String(String),
  Array(Vec<Box<Value>>),
  Object(Map<String, Box<Value>>)
}

pub struct Client {
  stream: TcpStream,
  addr: String,
  subscriptions: HashSet<Vec<String>>
}

impl Client {
  pub fn connect(addr: &str) -> Self {
    let stream = TcpStream::connect(addr).unwrap();
    Self { stream , addr: String::from(addr), subscriptions: HashSet::new() }
  }
  pub fn subscribe(&mut self, path: Vec<String>) {
    if !self.subscriptions.contains(&path) {
      self.subscriptions.insert(path);
    }
  }
  pub fn publish(&mut self, path: Vec<String>, value: Value){
    q
  }
}