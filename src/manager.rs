use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub fn fix_message(text: &[u8]) -> Vec<u8> {
    let mut message = text.to_owned();
    if let Some(last) = text.last() {
        // '\n'的u8形式=10
        if *last != 10 {
            message.push(10);
        }
    }
    message
}

pub struct TcpManager {
    pub peer_map: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
}

impl TcpManager {
    pub fn new() -> Self {
        Self {
            peer_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, addr: String, ax_stream: Arc<Mutex<TcpStream>>) {
        self.peer_map
            .clone()
            .lock()
            .unwrap()
            .insert(addr, ax_stream.clone());
    }

    pub fn send_message(&self, addr: String, text: &[u8]) {
        match self.peer_map.clone().lock().unwrap().get_mut(&addr) {
            Some(ax_stream) => {
                let _ = ax_stream
                    .clone()
                    .lock()
                    .unwrap()
                    .write_all(&fix_message(text));
            }
            None => (),
        }
    }

    pub fn broadcast(&self, text: &[u8]) {
        let temp = &fix_message(text);
        for (peer, ax_stream) in self.peer_map.clone().lock().unwrap().iter() {
            let mut message: Vec<u8> = Vec::new();
            message.extend_from_slice(format!("{}: ", peer).as_bytes());
            message.extend_from_slice(temp);
            let _ = ax_stream.clone().lock().unwrap().write_all(&message);
        }
    }

    pub fn remove(&self, addr: String) {
        self.peer_map.clone().lock().unwrap().remove(&addr);
    }
}
