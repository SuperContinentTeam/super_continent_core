use std::sync::Arc;

use crate::{
    state::state::{self, STATE_MAP},
    ws::{self, send_message, AXController}, db,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;

async fn join_room(message: Value, websocket: AXController) {
    let name = message.get("name").unwrap().as_str().unwrap();
    let room = message.get("room").unwrap().as_str().unwrap();
    println!("Player {} join the room: {}", name, room);

    let room_map_clone = STATE_MAP.clone();
    let mut room_map = room_map_clone.lock().await;

    match room_map.get_mut(room) {
        Some(state) => {
            let state = state.clone();
            let mut state = state.lock().await;

            if state.can_join() {
                state.players.push(name.to_string());

                db::update_room_info(room.to_string(), &json!({
                    "use_number": state.players.len() as u8,
                    "max_number": state.max_number,
                    "pause": state.pause,
                    "add_player": name
                })).await;

                ws::add_client(name.to_string(), websocket.clone()).await;
            }
        }
        None => {
            // 创建并运行 State 状态机
            let max_number = {
                match message.get("max_number") {
                    Some(v) => v.as_u64().unwrap() as u8,
                    None => 10,
                }
            };

            db::save_room_info(room.to_string(), db::RoomInfo{
                use_number: 1,
                max_number,
                pause: true,
                players: vec![name.to_string()]
            }).await;

            let mut s = state::State::new(name.to_string(), max_number);
            s.players.push(name.to_string());
            let ax_s = Arc::new(Mutex::new(s));

            room_map.insert(room.to_string(), ax_s.clone());
            tokio::task::spawn(state::run_state(ax_s.clone()));
        }
    }
    send_message(&json!({"status": "success"}), &websocket).await;
}

async fn query_rooms(websocket: AXController) {
    let result = db::query_all_rooms().await;
    send_message(&json!(result), &websocket).await;
}

pub async fn bypass(op: &str, message: Value, websocket: AXController) {
    match op {
        "join" => {
            join_room(message, websocket).await;
        }
        "query" => {
            if let Some(payload) = message.get("payload") {
                match payload.as_str().unwrap() {
                    "rooms" => {
                        query_rooms(websocket).await;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
