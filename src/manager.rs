use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref TCP_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    static ref MSG_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    // static ref STATE_MAP: HashMap<String, u8> = HashMap::new();
    static ref RELATION: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn tcp_manager_run() {
    println!("启动通信监听线程...");
    let mut tick = 1;
    let tcp_queue_clone = TCP_QUEUE.clone();

    thread::spawn(move || loop {
        if let Some(tcp_message) = tcp_queue_clone.lock().unwrap().pop_front() {
            println!("{}", tcp_message);
        }
        println!("通信运行时间: {}s", tick);
        tick += 1;
        thread::sleep(Duration::from_secs(1))
    });
}

pub fn state_manager_run() {
    println!("启动状态更新线程...");
    let mut tick = 1;
    let msg_queue_clone = MSG_QUEUE.clone();
    thread::spawn(move || loop {
        if let Some(msg_value) = msg_queue_clone.lock().unwrap().pop_front() {
            println!("{}", msg_value);
        }
        println!("状态更新时间: {}s", tick);
        tick += 1;
        thread::sleep(Duration::from_secs(1));
    });
}
