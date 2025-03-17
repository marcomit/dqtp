use std::{collections::HashSet, net::TcpStream, sync::Arc};

use serde_json::Value;

// use crate::broker::{Broker, BROKER};

pub struct Client {
  stream: TcpStream,
  subscription: HashSet<Vec<String>>
}

impl Client {
  pub fn new(stream: &mut TcpStream) -> Self {
    println!("New client");
    Self { stream: stream.try_clone().unwrap(), subscription: HashSet::new()  }
  }
  pub fn handle_connection(&mut self) {
    todo!()
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