// use std::sync::{Condvar, Mutex};

// #[derive(Debug, Clone)]
// pub struct MessageContent {
//     addr: String,
//     content: String,
//     count: i32,
// }

// impl MessageContent {
//     pub fn new(addr: String, content: String, count: i32) -> Self {
//         Self {
//             addr,
//             content,
//             count,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct MessageQueue {
//     pub queue: Mutex<Vec<MessageContent>>,
//     pub condvar: Condvar,
// }

// impl MessageQueue {
//     pub fn new() -> Self {
//         Self {
//             queue: Mutex::new(Vec::new()),
//             condvar: Condvar::new(),
//         }
//     }

//     pub fn send(&self, msg: MessageContent) {
//         let mut queue = self.queue.lock().unwrap();
//         queue.push(msg);
//         self.condvar.notify_one();
//     }

//     pub fn recv(&self) -> MessageContent {
//         let mut queue = self.queue.lock().unwrap();
//         while queue.is_empty() {
//             queue = self.condvar.wait(queue).unwrap()
//         }
//         queue.remove(0)
//     }
// }
