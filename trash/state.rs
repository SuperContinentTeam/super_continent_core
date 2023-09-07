use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;

use crate::tcp;

lazy_static! {
    pub static ref STATE_MAP: Arc<Mutex<HashMap<String, Arc<Mutex<State>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct State {
    pub name: String,
    pub tick: usize,
    pub time_flow: usize,
    pub tcp_list: Vec<Arc<Mutex<TcpStream>>>,
    pub running: bool,
}

impl State {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tick: 0,
            time_flow: 1,
            tcp_list: Vec::new(),
            running: true,
        }
    }

    pub fn next_tick(&mut self) {
        self.tick += self.time_flow;
    }

    pub fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "tick": self.tick
        })
    }

    pub fn distribution(&mut self) {
        let data = self.serialize();
        let msg = Arc::new(tcp::fix_message(data.to_string()));

        for tcp in self.tcp_list.iter_mut() {
            let mutex_tcp_clone = tcp.clone();
            let msg_clone = msg.clone();
            // 为每一个发送消息的动作开辟一个线程
            thread::spawn(move || {
                let mut tcp = mutex_tcp_clone.lock().unwrap();
                let _ = tcp.write_all(&msg_clone);
            });
        }
    }
}

pub fn init() {
    let mutex_state_clone = STATE_MAP.clone();
    let mut state_map = mutex_state_clone.lock().unwrap();

    let room_name = "localhost".to_string();
    let state = Arc::new(Mutex::new(State::new(room_name.clone())));
    state_map.insert(room_name.clone(), state.clone());
    thread::spawn(move || {
        state_threading_start(state);
    });
}

pub fn state_threading_start(mutex_state: Arc<Mutex<State>>) {
    let mutex_state_clone = mutex_state.clone();
    loop {
        let mut state = mutex_state_clone.lock().unwrap();
        if !state.running {
            return;
        }

        thread::sleep(Duration::from_secs(1));
        println!(
            "State[{}], tcp number: {}",
            state.name,
            state.tcp_list.len()
        );
        if state.tcp_list.len() > 0 {
            state.distribution();
            state.next_tick();
        }
    }
}
