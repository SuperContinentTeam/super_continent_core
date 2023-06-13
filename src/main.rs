mod commander;
mod manager;
mod message;
mod reference;
mod state;
mod tcp;

use dotenv::dotenv;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::commander::Command;
use crate::reference::get_string;

fn main() {
    dotenv().ok();

    let listener = TcpListener::bind("0.0.0.0:55555").unwrap();
    println!("Listening 0.0.0.0:55555...");

    manager::initial();
    tcp::tcp_manager_run();
    state::state_manager_run();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // 创建新线程处理客户端连接
                let addr = s.peer_addr().unwrap().to_string();
                let arc_mutex_stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(s));
                // thread::spawn(move || handle_socket(peer_addr, arc_mutex_stream.clone()));
                thread::spawn(move || handle_tcp(addr, arc_mutex_stream.clone()));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_tcp(addr: String, ax_tcp: Arc<Mutex<TcpStream>>) {
    println!("{} is connected", addr);
    let mut buffer = [0; 1024];
    let mut received_data = String::new();
    loop {
        let recv = ax_tcp.clone().lock().unwrap().read(&mut buffer);

        match recv {
            Ok(bytes_read) if bytes_read > 0 => {
                // 将接收到的数据转换为字符串
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                // 将接收到的数据拼接到已接收数据的末尾
                received_data.push_str(&data);

                if received_data.ends_with("\n") {
                    let body = received_data.clone();
                    received_data.clear();

                    // 转化为json
                    let body: serde_json::Value = serde_json::from_str(body.trim_end()).unwrap();
                    // 转化为指令实例
                    let command = Command {
                        op: get_string(body.get("op").unwrap()),
                        body: body.get("body").unwrap().to_owned(),
                    };
                    commander::command_executor(command);
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

// fn handle_socket(peer_addr: String, ax_stream: Arc<Mutex<TcpStream>>) {
//     let mut buffer = [0; 1024];
//     let mut received_data = String::new();
//     println!("已连接: {}", peer_addr);

//     loop {
//         let recv = ax_stream.clone().lock().unwrap().read(&mut buffer);

//         match recv {
//             Ok(bytes_read) if bytes_read > 0 => {
//                 // 将接收到的数据转换为字符串
//                 let data = String::from_utf8_lossy(&buffer[..bytes_read]);
//                 // 将接收到的数据拼接到已接收数据的末尾
//                 received_data.push_str(&data);

//                 if received_data.ends_with("\n") {
//                     let body = received_data.clone();
//                     received_data.clear();

//                     // 转化为json
//                     let message: serde_json::Value = serde_json::from_str(body.trim_end()).unwrap();
//                     if let Some(op) = message.get("op") {
//                         let str_op = op.as_str().unwrap();
//                         println!("获取到的操作: {}", str_op);
//                         match str_op {
//                             "join" => {
//                                 let name = message.get("name").unwrap().to_string();
//                                 let room = message.get("room").unwrap().to_string();
//                             }
//                             _ => (),
//                         }
//                     }
//                     // tcp_manager::send_message(ax_stream.clone(), body.as_bytes());
//                 }

//                 continue;
//             }
//             Ok(_) => {
//                 println!("Disconnect socket");
//                 break;
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 break;
//             }
//         }
//     }
// }

// fn parse_data(message: String) -> serde_json::Value {
//     let msg = message.trim_end();
//     serde_json::from_str(msg).unwrap()
// }
