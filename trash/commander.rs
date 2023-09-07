// 注意: 整个命令执行器都是运行在tcp的消息接收线程之中

use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{reference::get_string, state, tcp};
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    pub op: String,
    pub body: Value,
}

pub fn command_executor(tcp: &mut tcp::Tcp, c: Command) {
    let body = c.body;
    let op = c.op.as_str();

    let no_state = tcp.state.is_none();

    if no_state && op != "join" {
        println!("必须加入房间才能使用指令");
        return;
    }

    match op {
        "join" => {
            if no_state {
                join(tcp, body);
            }
        }
        _ => {
            println!("Not found command: {}", op);
        }
    }
}

pub fn join(tcp: &mut tcp::Tcp, body: Value) {
    let name = get_string(body.get("name").unwrap());
    let room = get_string(body.get("room").unwrap());
    println!("玩家【{}】加入房间[{}]", name, room);

    let state_map_clone = state::STATE_MAP.clone();
    let mut state_map = state_map_clone.lock().unwrap();

    let mutex_state = match state_map.get(&room) {
        Some(s) => s.clone(),
        None => {
            let s = Arc::new(Mutex::new(state::State::new(room.clone())));
            state_map.insert(room, s.clone());
            // 新增的状态机启动一个新的线程
            let threading_state = s.clone();
            thread::spawn(move || {
                state::state_threading_start(threading_state);
            });

            s.clone()
        }
    };

    println!("{}", 3);
    let mut state = mutex_state.lock().unwrap();
    println!("{}", 4);
    println!("{:#?}", state.tcp_list);
    state.tcp_list.push(tcp.stream.clone());
    println!("{:#?}", state.tcp_list);
    tcp.state = Some(mutex_state.clone());
}
