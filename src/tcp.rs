use std::io::Read;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{commander, reference, state};

pub struct Tcp {
    pub state: Option<Arc<Mutex<state::State>>>,
    pub addr: String,
    pub stream: Arc<Mutex<TcpStream>>,
}

pub fn fix_message(text: String) -> Vec<u8> {
    let text = text.as_bytes();
    let mut msg = text.to_owned();
    if let Some(last) = msg.last() {
        if *last != 10 {
            msg.push(10);
        }
    }
    msg
}

pub fn receive_threading_start(tcp: &mut Tcp) {
    println!("{} is connected", tcp.addr);
    let mut buffer = [0; 1024];
    let mut received_data = String::new();
    loop {
        let mutex_tcp_clone = tcp.stream.clone();
        let mut stream = mutex_tcp_clone.lock().unwrap();

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                // 将接收到的数据转换为字符串
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                received_data.push_str(&data);
                if !received_data.ends_with("\n") {
                    continue;
                }

                let body = received_data.clone();
                received_data.clear();

                let body: serde_json::Value = serde_json::from_str(body.trim_end()).unwrap();
                // 转化为指令实例
                let command = commander::Command {
                    op: reference::get_string(body.get("op").unwrap()),
                    body: body.get("body").unwrap().to_owned(),
                };

                commander::command_executor(tcp, command);
            }
            Err(e) => {
                println!("Disconnect socket, Error: {}", e);
                break;
            }
        }
    }
}
