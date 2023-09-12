use std::{collections::HashMap, sync::Arc};

use serde_json::{json, Value};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use crate::{
    state::state::{self, AXState},
    ws::{send_message, AXController, self},
};
// 房间表
pub type RoomMap = Arc<Mutex<HashMap<String, AXState>>>;
lazy_static!{
    pub static ref ROOM_MAP: RoomMap = RoomMap::default();
}

async fn join_room(message: Value, websocket: AXController) {
    let name = message.get("name").unwrap().as_str().unwrap();
    let room = message.get("room").unwrap().as_str().unwrap();
    println!("Player {} join the room: {}", name, room);

    let room_map_clone = ROOM_MAP.clone();
    let mut room_map = room_map_clone.lock().await;

    match room_map.get_mut(room) {
        Some(state) => {
            let state = state.clone();
            let mut state = state.lock().await;

            if state.can_join() {
                state.players.push(name.to_string());
                ws::add_client(name.to_string(), websocket.clone()).await;
            }
        }
        None => {
            // 创建并运行 State 状态机
            let s = state::add_state(message.clone()).await;
            
            ws::add_client(name.to_string(), websocket.clone()).await;
            room_map.insert(room.to_string(), s.clone());
        }
    }

    send_message(&json!({"status": "success"}), &websocket).await;
}

// async fn query_rooms(websocket: AXController) {
//     let peer_map_clone = PEER_MAP.clone();
//     let peer_map = peer_map_clone.lock().await;

//     let state_map_clone = STATE_MAP.clone();
//     let state_map = state_map_clone.lock().unwrap();
//     let mut result: HashMap<String, Value> = HashMap::new();
//     for (peer, state) in peer_map.iter().zip(state_map.iter()) {
//         let name = peer.0.to_string();
//         let use_number = peer.1.len();

//         let s_clone = state.1.clone();
//         let s = s_clone.lock().unwrap();
//         let max_number = s.max_number;
//         result.insert(
//             name,
//             json!({
//                 "useNumber": use_number,
//                 "maxNumber": max_number
//             }),
//         );
//     }

//     send_message(&json!(result), &websocket).await;
// }

pub async fn bypass(op: &str, message: Value, websocket: AXController) {
    match op {
        "join" => {
            join_room(message, websocket).await;
        }
        // "query" => {
        //     if let Some(payload) = message.get("payload") {
        //         match payload.as_str().unwrap() {
        //             "rooms" => {
        //                 query_rooms(websocket).await;
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        _ => {}
    }
}
