// 注意: 整个命令执行器都是运行在tcp的消息接收线程之中

use crate::{reference::get_string, tcp};
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    pub op: String,
    pub body: Value,
}

pub fn command_executor(tcp: &tcp::Tcp, c: Command) {
    let body = c.body;
    let op = c.op.as_str();

    if tcp.state.is_none() && op != "join" {
        println!("必须加入房间才能使用指令");
        return;
    }

    match op {
        "join" => join(body),
        _ => {
            println!("Not found command: {}", op);
        }
    }
}

pub fn join(body: Value) {
    let name = get_string(body.get("name").unwrap());
    let room = get_string(body.get("room").unwrap());
    println!("玩家【{}】加入房间[{}]", name, room);

    // let mut m = relation_clone.lock().unwrap();
    // let _ = match m.get_mut(&room) {
    //     Some(tcp_set) => {
    //         tcp_set.insert(name);
    //     }
    //     None => {
    //         let mut tcp_set = HashSet::new();
    //         tcp_set.insert(name);
    //         m.insert(room, tcp_set);
    //     }
    // };
    // println!("{:#?}", m);
}
