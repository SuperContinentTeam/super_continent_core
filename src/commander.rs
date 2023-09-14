use std::sync::Arc;

use crate::{
    db::{self, USER_IN_ROOM},
    reference::{AxClient, PEER_USER_MAP},
    state::state::{run_state, State, STATE_MAP},
    ws::send_message,
};
use serde_json::json;
use tokio::sync::Mutex;
use tungstenite::Message;

pub async fn join_room(room: &str, name: &str, client: AxClient) {
    println!("Player {} join the room: {}", name, room);
    let c = client.lock().await;
    let mut room_map = STATE_MAP.lock().await;

    let st = {
        match room_map.get_mut(room) {
            Some(s) => s.clone(),
            None => {
                let max_number = 10;
                // 缓存
                db::save_room_info(
                    room,
                    db::RoomInfo {
                        use_number: 0, // 设为0 为了降低代码重复性 在后面采用更新的方式写入1
                        max_number,
                        status: 0,
                        players: vec![name.to_string()],
                    },
                )
                .await;

                // 创建并运行 State 状态机
                let s = State::new(room.to_string(), max_number, 10);
                let ax_s = Arc::new(Mutex::new(s));

                tokio::task::spawn(run_state(ax_s.clone()));
                room_map.insert(room.to_string(), ax_s.clone());

                ax_s
            }
        }
    };

    let mut s = st.lock().await;

    let can_join: i32 = s.can_join(name);
    if can_join != 0 {
        let _ = c.tx.unbounded_send(Message::Text(can_join.to_string()));
        return;
    }

    s.add_player(name);
    db::update_room_info(
        name,
        &json!({"use_number": s.players.len(), "add_player": name}),
    )
    .await;

    // 保存用户与连接ip的对应关系
    PEER_USER_MAP
        .lock()
        .await
        .insert(name.to_string(), c.addr.clone());

    // 保存用户与房间的对应关系 方便查询
    USER_IN_ROOM
        .lock()
        .await
        .insert(name.to_string(), room.to_string());
}

pub async fn update_status(room: &str, status: &str) {
    let mut room_map = STATE_MAP.lock().await;
    if let Ok(s) = status.parse::<i32>() {
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
