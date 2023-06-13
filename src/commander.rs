use std::collections::HashSet;

use crate::{manager, reference::get_string};
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    pub op: String,
    pub body: Value,
}

pub fn command_executor(c: Command) {
    let body = c.body;
    let op = c.op.as_str();
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

    let relation_clone = manager::RELATION.clone();
    let mut m = relation_clone.lock().unwrap();
    let _ = match m.get_mut(&room) {
        Some(tcp_set) => {
            tcp_set.insert(name);
        }
        None => {
            let mut tcp_set = HashSet::new();
            tcp_set.insert(name);
            m.insert(room, tcp_set);
        }
    };
    println!("{:#?}", m);
}
