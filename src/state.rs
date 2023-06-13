use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref STATE_MAP: HashMap<String, Arc<Mutex<State>>> = HashMap::new();
}

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
