use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::state;

lazy_static! {
    pub static ref TCP_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    pub static ref MSG_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    pub static ref RELATION: Arc<Mutex<HashMap<String, Arc<Mutex<HashSet<String>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn tcp_manager_run() {
    println!("启动通信对立消费线程...");
    let mut tick = 1;
    let tcp_queue_clone = TCP_QUEUE.clone();

    thread::spawn(move || loop {
        if let Some(tcp_message) = tcp_queue_clone.lock().unwrap().pop_front() {
            println!("{}", tcp_message);
        }
        tick += 1;
        thread::sleep(Duration::from_secs(1))
    });
}

pub fn state_manager_run() {
    println!("启动状态队列消费线程...");
    let mut tick = 1;
    let msg_queue_clone = MSG_QUEUE.clone();
    thread::spawn(move || loop {
        if let Some(msg_value) = msg_queue_clone.lock().unwrap().pop_front() {
            println!("{}", msg_value);
        }
        tick += 1;
        thread::sleep(Duration::from_secs(1));
    });
}

pub fn join_state(name: String, room: String) {
    let name_clone = name.clone();

    let mut temp_map = RELATION.lock().unwrap();

    let _ = match temp_map.get_mut(&name_clone) {
        Some(tcp_set) => {
            tcp_set.clone().lock().unwrap().insert(name_clone.clone());
        }
        None => {
            let mut tcp_set = HashSet::new();
            tcp_set.insert(name_clone.clone());
            temp_map.insert(room.clone(), Arc::new(Mutex::new(tcp_set)));
        }
    };
}
