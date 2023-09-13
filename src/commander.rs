use std::sync::Arc;

use crate::{
    db,
    state::state::{self, STATE_MAP},
    ws::{send_message, AxClient, PEER_USER_MAP},
};
use serde_json::{json, Value};
use tokio::sync::Mutex;

pub async fn join_room(message: Value, client: AxClient) {
    let name = message.get("name").unwrap().as_str().unwrap();
    let room = message.get("room").unwrap().as_str().unwrap();
    println!("Player {} join the room: {}", name, room);

    let client = client.lock().await;

    let mut room_map = STATE_MAP.lock().await;
    match room_map.get_mut(room) {
        Some(s) => {
            let s_clone = s.clone();
            let mut ax_s = s_clone.lock().await;
            if ax_s.can_join() {
                ax_s.players.push(name.to_string());
                db::update_room_info(
                    name,
                    &json!({
                        "use_number": ax_s.players.len() as u8,
                        "max_number": ax_s.max_number,
                        "pause": ax_s.pause,
                        "add_player": name
                    }),
                )
                .await;
                PEER_USER_MAP
                    .lock()
                    .await
                    .insert(name.to_string(), client.addr.clone());
            };
        }
        None => {
            // 创建并运行 State 状态机
            let max_number = {
                match message.get("max_number") {
                    Some(v) => v.as_u64().unwrap() as u8,
                    None => 10,
                }
            };

            db::save_room_info(
                room,
                db::RoomInfo {
                    use_number: 1,
                    max_number,
                    pause: true,
                    players: vec![name.to_string()],
                },
            )
            .await;

            let mut s = state::State::new(name.to_string(), max_number);
            s.players.push(name.to_string());
            let ax_s = Arc::new(Mutex::new(s));

            PEER_USER_MAP
                .lock()
                .await
                .insert(name.to_string(), client.addr.clone());

            room_map.insert(room.to_string(), ax_s.clone());
            tokio::task::spawn(state::run_state(ax_s.clone()));
        }
    }
}

pub async fn update_state(message: Value) {
    let room = message.get("room").unwrap().as_str().unwrap();
    let room_map_clone = STATE_MAP.clone();
    let mut room_map = room_map_clone.lock().await;
    if let Some(s) = room_map.get_mut(room) {
        let s_clone = s.clone();
        let mut s = s_clone.lock().await;

        if let Some(v) = message.get("pause") {
            s.pause = v.as_bool().unwrap();
        }
    }
}

pub async fn bypass(op: &str, message: Value, client: AxClient) {
    match op {
        "join" => join_room(message, client).await,
        "query" => {
            if let Some(payload) = message.get("payload") {
                match payload.as_str().unwrap() {
                    "rooms" => {
                        let result = db::query_all_rooms().await;
                        send_message(&json!(result), client).await;
                    }
                    _ => {}
                }
            }
        }
        "update" => update_state(message).await,
        _ => {}
    }
}
