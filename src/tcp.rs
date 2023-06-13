use std::{io::Write, net::TcpStream};

use crate::message::Message;

pub struct Tcp {
    pub name: String,
    pub stream: TcpStream,
}

impl Tcp {
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
