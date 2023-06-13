use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub struct State {
    pub name: String,
    pub tick: usize,
    pub tcp_list: Vec<Arc<Mutex<TcpStream>>>,
}

impl State {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tick: 0,
            tcp_list: Vec::new(),
        }
    }
}
