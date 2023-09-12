use std::{collections::HashMap, sync::Arc};
use serde::Serialize;
use lazy_static::lazy_static;
use serde_json::{Value, json};
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct RoomInfo {
    pub use_number: u8,
    pub max_number: u8,
    pub pause: bool,
    pub players: Vec<String>
}

type RoomInfoMap = Arc<Mutex<HashMap<String, RoomInfo>>>;

lazy_static! {
    pub static ref DB: RoomInfoMap = RoomInfoMap::default();
}

pub async fn save_room_info(name: String, info: RoomInfo) {
    let db_clone = DB.clone();
    let mut db = db_clone.lock().await;
    if !db.contains_key(&name) {
        db.insert(name, info);
    }
}

pub async fn query_all_rooms() -> Value {
    let db_clone = DB.clone();
    let db = db_clone.lock().await;

    json!(*db)
}

pub async fn update_room_info(name: String, value: &Value) {
    let db_clone = DB.clone();
    let mut db = db_clone.lock().await;
    let room_info = db.get_mut(&name);
    if room_info.is_none() {
        return;
    }
    let room_info = room_info.unwrap();

    if let Some(v) = value.get("use_number") {
        room_info.use_number = v.as_u64().unwrap() as u8;
    }

    if let Some(v) = value.get("max_number") {
        room_info.max_number = v.as_u64().unwrap() as u8;
    }

    if let Some(v) = value.get("pause") {
        room_info.pause = v.as_bool().unwrap();
    }

    if let Some(v) = value.get("add_player") {
        room_info.players.push(v.as_str().unwrap().to_string());
    }

    if let Some(v) = value.get("remove_player") {
        let p = v.as_str().unwrap();
        room_info.players.retain(|x| x.as_str() != p);
    }
}