use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::{manager, message, tcp};

lazy_static! {
    pub static ref MSG_QUEUE: Arc<Mutex<VecDeque<message::Message>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    pub static ref STATE_MAP: Arc<Mutex<HashMap<String, GameState>>> =
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
        GameState::new("localhost".to_string()),
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

    pub fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "tick": self.tick
        })
    }
}

pub fn state_manager_run() {
    println!("Launch State Threading...");

    // let msg_queue_clone = MSG_QUEUE.clone();
    thread::spawn(move || loop {
        // 获取所有已存在的状态机
        let state_map_clone = STATE_MAP.clone();
        let mut state_map = state_map_clone.lock().unwrap();

        // 获取状态机对应的Tcp连接
        let relateion_clone = manager::RELATION.clone();
        let relateion = relateion_clone.lock().unwrap();

        // 批量生成消息
        let mut messages: Vec<message::Message> = Vec::new();
        for (state_name, state) in state_map.iter() {
            for tcp_name in relateion.get(state_name).unwrap() {
                messages.push(message::Message {
                    sender: state_name.clone(),
                    receiver: tcp_name.clone(),
                    body: state.serialize(),
                });
            }
        }
        //发送消息给TCP线程
        tcp::add_messages_for_tcp(messages);

        // 计算所有状态机的下一个状态
        for state in state_map.values_mut() {
            state.next();
        }

        // 检查消息队列 更新状态机中的参数
        // if let Some(msg_value) = msg_queue_clone.lock().unwrap().pop_front() {
        //     println!("{:#?}", msg_value);
        // }

        thread::sleep(Duration::from_secs(1));
    });
}

// pub fn add_message_for_state(message: message::Message) {
//     let msg_queue_clone = MSG_QUEUE.clone();
//     let mut queue = msg_queue_clone.lock().unwrap();
//     queue.push_back(message);
// }
