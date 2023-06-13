use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{io::Write, net::TcpStream};

lazy_static! {
    pub static ref TCP_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
}

use crate::message::Message;

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
        if let Some(tcp_message) = tcp_queue_clone.lock().unwrap().pop_front() {
            println!("{}", tcp_message);
        }

        thread::sleep(Duration::from_secs(1))
    });
}
