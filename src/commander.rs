use crate::reference::get_string;
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
}
