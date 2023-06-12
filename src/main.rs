use lazy_static::lazy_static;
use message_manager::MessageManager;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

lazy_static! {
    static ref MANAGER: MessageManager = MessageManager::new();
}

mod message_manager;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:55555").unwrap();
    println!("Listening 0.0.0.0:55555...");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // 创建新线程处理客户端连接
                let addr = s.peer_addr().unwrap().to_string();
                let arc_mutex_stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(s));
                let peer_addr = addr.clone();
                // 保存线程对象
                MANAGER.insert(addr, arc_mutex_stream.clone());
                thread::spawn(move || handle_socket(peer_addr, arc_mutex_stream.clone()));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_socket(peer_addr: String, ax_stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 1024];
    let mut received_data = String::new();
    println!("已连接: {}", peer_addr);

    loop {
        let recv = ax_stream.clone().lock().unwrap().read(&mut buffer);

        match recv {
            Ok(bytes_read) if bytes_read > 0 => {
                // 将接收到的数据转换为字符串
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                // 将接收到的数据拼接到已接收数据的末尾
                received_data.push_str(&data);
                if received_data.ends_with("\n") {
                    let body = received_data.clone();
                    println!("接收信息为, {:#?}: {:#?}", peer_addr, body);
                    received_data.clear();
                    MANAGER.broadcast(body.as_bytes());
                    // broadcast(body.as_bytes());
                }

                continue;
            }
            Ok(_) => {
                println!("Disconnect socket");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

// fn parse_data(message: String) -> serde_json::Value {
//     let msg = message.trim_end();
//     serde_json::from_str(msg).unwrap()
// }
