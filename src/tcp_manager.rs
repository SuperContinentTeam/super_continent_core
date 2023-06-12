use lazy_static::lazy_static;
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

lazy_static! {
    static ref TCP_MAP: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn insert(addr: String, ax_stream: Arc<Mutex<TcpStream>>) {
    TCP_MAP
        .clone()
        .lock()
        .unwrap()
        .insert(addr, ax_stream.clone());
}

pub fn send_message(ax_stream: Arc<Mutex<TcpStream>>, text: &[u8]) {
    let _ = ax_stream
        .clone()
        .lock()
        .unwrap()
        .write_all(&fix_message(text));
}
