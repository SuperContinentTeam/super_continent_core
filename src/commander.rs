use std::sync::Arc;

use crate::{
    db,
    state::state::{self, STATE_MAP},
    ws::{send_message, AxClient, PEER_USER_MAP},
};
use serde_json::json;
use tokio::sync::Mutex;

pub async fn join_room(room: &str, name: &str, client: AxClient) {
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
            let max_number = 10;

            let rf = db::RoomInfo {
                use_number: 1,
                max_number,
                status: 0,
                players: vec![name.to_string()],
            };
            db::save_room_info(room, rf).await;

            // 创建并运行 State 状态机
            let mut s = state::State::new(room.to_string(), max_number);
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

pub async fn update_status(room: &str, status: &str) {
    let mut room_map = STATE_MAP.lock().await;
    if let Ok(s) = status.parse::<u8>() {
        if let Some(r) = room_map.get_mut(room) {
            let mut r = r.lock().await;
            r.status = s;
        }
    }
}

pub async fn bypass_binary(options: &str, client: AxClient) {
    let cmd = options.split(";").collect::<Vec<&str>>();
    println!("{:?}", cmd);
    match cmd[0] {
        "01" => join_room(cmd[1], cmd[2], client).await,
        "02" => match cmd[1] {
            "rooms" => {
                let result = db::query_all_rooms().await;
                send_message(json!(result), client).await;
            }
            _ => {}
        },
        "03" => update_status(cmd[1], cmd[2]).await,
        _ => {}
    }
}
