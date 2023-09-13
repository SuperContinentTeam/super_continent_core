use std::sync::Arc;

use crate::{
    db::{self, USER_IN_ROOM},
    state::state::{self, STATE_MAP},
    ws::{send_message, AxClient, PEER_USER_MAP},
};
use serde_json::json;
use tokio::sync::Mutex;
use tungstenite::Message;

pub async fn join_room(room: &str, name: &str, client: AxClient) {
    println!("Player {} join the room: {}", name, room);
    let c = client.lock().await;
    let mut room_map = STATE_MAP.lock().await;

    match room_map.get_mut(room) {
        Some(s) => {
            let s_clone = s.clone();
            let mut ax_s = s_clone.lock().await;

            let can_join = ax_s.can_join(name.to_string());
            match can_join {
                0 => {
                    ax_s.players.push(name.to_string());

                    let use_number = ax_s.players.len() as u8;
                    db::update_room_info(
                        name,
                        &json!({"use_number": use_number, "add_player": name}),
                    )
                    .await;

                    // 保存用户与连接ip的对应关系
                    PEER_USER_MAP
                        .lock()
                        .await
                        .insert(name.to_string(), c.addr.clone());

                    // 保存用户与房间的对应关系 方便查询
                    USER_IN_ROOM.lock().await.insert(name.to_string(), room.to_string());
                }
                _ => {
                    let _ = c.tx.unbounded_send(Message::Text(can_join.to_string()));
                }
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
                .insert(name.to_string(), c.addr.clone());
            USER_IN_ROOM.lock().await.insert(name.to_string(), room.to_string());

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
                send_message(result.to_string(), client).await;
            }
            _ => {}
        },
        "03" => update_status(cmd[1], cmd[2]).await,
        _ => {}
    }
}
