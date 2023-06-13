use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    pub op: String,
    pub body: Value,
}

pub fn command_executor(c: Command) {
    // let body = c.body;
    match c.op.as_str() {
        "join" => {
            println!("join");
        }
        _ => {
            println!("Not found command: {}", c.op);
        }
    }
}

pub fn join(body: Value) {
    println!("{}", 1);
    let name = body.get("name").unwrap().to_string();
    let room = body.get("room").unwrap().to_string();
    println!("玩家【{}】加入房间[{}]", name, room);
}
