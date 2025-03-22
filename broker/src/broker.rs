use std::sync::{Arc, Mutex, MutexGuard};

use lazy_static::lazy_static;

use crate::client::Client;

use serde_json::Value;

pub struct Broker {
    pub clients: Arc<Mutex<Vec<Client>>>,
    // tree: Map<String, Client>,
    value: Value,
}

lazy_static! {
    pub static ref BROKER: Arc<Mutex<Broker>> = Arc::new(Mutex::new(Broker {
        clients: Arc::new(Mutex::new(vec![])),
        value: Value::Null
    }));
}

impl Broker {
    pub fn get() -> MutexGuard<'static, Broker> {
        BROKER.lock().unwrap()
    }
    pub fn add_client(&mut self, client: Client) {
        let clients = Arc::clone(&self.clients);
        let mut clients = clients.lock().unwrap();
        clients.push(client);
        println!("Aggiunto client");
    }
    pub fn handle(&mut self, client_id: String, buf: &[u8]) {
        println!("Handle {}", client_id);
        let clients = Arc::clone(&self.clients);
        let mut clients = clients.lock().unwrap();

        let mut to_remove = Vec::new();

        for (index, client) in clients.iter_mut().enumerate() {
            println!("For handle");
            // if let Some(id) = client.get_id() {
            //     if id == client_id {
            //         continue;
            //     }

            match client.send(buf) {
                Ok(_) => println!("Sent to client"),
                Err(e) => {
                    println!("Failed to send to client: {}", e);
                    to_remove.push(index);
                } // }
            }
        }

        // Remove disconnected clients (in reverse order to maintain indices)
        for index in to_remove.iter().rev() {
            clients.remove(*index);
        }
    }

    // pub fn get_clients(&self) {
    //     let clients = Arc::clone(&self.clients);
    //     let mut clients = clients.lock().unwrap();
    // }

    // pub fn get_from_path(&self, path: Vec<String>) -> Value {
    //     let mut current = self.value.clone();

    //     for i in 0..path.len() - 1 {
    //         let p = &path[i];
    //         current = self.validate_path(p.to_string(), current)
    //     }
    //     current
    // }

    // pub fn modify_from_path(&mut self, path: Vec<String>, value: Value) {
    //     if path.len() == 0 {
    //         return;
    //     }

    //     let mut path = path.clone();

    //     let last = path.pop().unwrap();

    //     let last_value = self.get_from_path(path);

    //     use Value::*;
    //     match last_value {
    //         Array(mut v) => {
    //             if let Ok(index) = last.parse::<usize>() {
    //                 if index > v.len() {
    //                     return;
    //                 }
    //                 v[index] = value;
    //             }
    //         }
    //         Object(mut map) => {
    //             map.insert(last, value);
    //         }
    //         _ => {}
    //     }
    // }
    // fn validate_path(&self, path: String, value: Value) -> Value {
    //     use Value::*;

    //     match value {
    //         Array(v) => {
    //             if let Ok(index) = path.parse::<usize>() {
    //                 if index > v.len() {
    //                     return Null;
    //                 }
    //                 return v[index].clone();
    //             }
    //             Null
    //         }
    //         Object(map) => match map.get(&path) {
    //             Some(v) => v.clone(),
    //             None => Null,
    //         },
    //         _ => Null,
    //     }
    // }
}
