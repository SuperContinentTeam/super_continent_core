use crate::message::Message;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{io::Write, net::TcpStream};
lazy_static! {
    pub static ref TCP_QUEUE: Arc<Mutex<VecDeque<Message>>> = Arc::new(Mutex::new(VecDeque::new()));
}

pub struct Tcp {
    pub name: String,
    pub stream: TcpStream,
}

impl Tcp {
    #[allow(dead_code)]
    pub fn send(&mut self, message: Message) {
        let data = message.to_json();
        let binding = data.to_string();
        let msg = binding.as_bytes();
        let mut body = msg.to_owned();

        if let Some(last) = body.last() {
            if *last != 10 {
                body.push(10);
            }
        }
        let _ = self.stream.write_all(&body);
    }
}

pub fn tcp_manager_run() {
    println!("Launch TCP Threading ...");

    let tcp_queue_clone = TCP_QUEUE.clone();

    thread::spawn(move || loop {
        // 获取所有的Tcp连接

        // 获取所有的状态机的当前状态

        // 分发状态给所有Tcp连接

        // 检查是否有状态机处理消息后的回执
        if let Some(tcp_message) = tcp_queue_clone.lock().unwrap().pop_front() {
            println!("{:#?}", tcp_message);
        }

        thread::sleep(Duration::from_secs(1))
    });
}

#[allow(dead_code)]
pub fn add_message_for_tcp(sender: String, receiver: String, body: serde_json::Value) {
    let queue_clone = TCP_QUEUE.clone();
    let mut queue = queue_clone.lock().unwrap();
    let message: Message = Message {
        sender,
        receiver,
        body,
    };
    queue.push_back(message);
}

pub fn add_messages_for_tcp(messages: Vec<Message>) {
    let queue_clone = TCP_QUEUE.clone();
    let mut queue = queue_clone.lock().unwrap();
    queue.extend(messages);
}
