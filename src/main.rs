use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:55555").unwrap();
    println!("Listening 0.0.0.0:55555...");
    // let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 创建新线程处理客户端连接
                thread::spawn(move || handle_socket(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_socket(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut received_data = String::new();
    let peer_addr = stream.peer_addr().unwrap();
    println!("已连接: {}", peer_addr);

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                // 将接收到的数据转换为字符串
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);

                // 将接收到的数据拼接到已接收数据的末尾
                received_data.push_str(&data);
                if received_data.ends_with("\n") {
                    println!("接收信息为: {}", received_data);
                    received_data.clear();
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
