use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::message;

lazy_static! {
    pub static ref MSG_QUEUE: Arc<Mutex<VecDeque<message::Message>>> =
        Arc::new(Mutex::new(VecDeque::new()));
}

lazy_static! {
    pub static ref STATE_MAP: Arc<Mutex<HashMap<String, Arc<Mutex<GameState>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct GameState {
    pub name: String,
    pub tick: usize,
    pub pause: bool,
}

pub fn initial() {
    let _ = STATE_MAP.clone().lock().unwrap().insert(
        "localhost".to_string(),
        Arc::new(Mutex::new(GameState::new("localhost".to_string()))),
    );
}

impl GameState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tick: 0,
            pause: false,
        }
    }

    pub fn next(&mut self) {
        if self.pause {
            return;
        }

        self.tick += 1;
    }
}

pub fn state_manager_run() {
    println!("Launch State Threading...");

    let msg_queue_clone = MSG_QUEUE.clone();
    thread::spawn(move || loop {
        if let Some(msg_value) = msg_queue_clone.lock().unwrap().pop_front() {
            println!("{:#?}", msg_value);
        }

        thread::sleep(Duration::from_secs(1));
    });
}
