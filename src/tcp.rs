use std::io::{Read, Write};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::state;

pub struct Tcp {
    pub state: Option<Arc<Mutex<state::State>>>,
    pub addr: String,
    pub stream: Arc<Mutex<TcpStream>>,
}

pub fn receive_threading_start(tcp: Tcp) {
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
                let _ = stream.write_all(body.as_bytes());
            }
            Err(e) => {
                println!("Disconnect socket, Error: {}", e);
                break;
            }
        }
    }
}
