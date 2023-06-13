use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::state;

lazy_static! {
    pub static ref TCP_QUEUE: Arc<Mutex<VecDeque<serde_json::Value>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    pub static ref RELATION: Arc<Mutex<HashMap<String, HashSet<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn initial() {
    let x = std::env::var("CREATE_LOCAL_ROOM").unwrap_or("1".to_owned());
    if x.as_str() == "1" {
        // 创建一个本地房间
        state::initial();

        // Tcp连接与State房间的对应关系表
        let _ = RELATION
            .clone()
            .lock()
            .unwrap()
            .insert("localhost".to_string(), HashSet::new());
    }
}
