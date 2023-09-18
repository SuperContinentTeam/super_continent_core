use crate::reference::STATE_MAP;
use crate::reference::{AxClient, PEER_USER_MAP};

pub async fn join_room(room: &str, name: &str, client: AxClient) {
    println!("Player {} join the room: {}", name, room);
    let c = client.lock().await;
    let mut st_map = STATE_MAP.lock().await;
    let st = st_map.get_mut(room).unwrap();
    let mut s = st.lock().await;

    s.add_player(name);

    // 保存用户与连接ip的对应关系
    PEER_USER_MAP
        .lock()
        .await
        .insert(name.to_string(), c.addr.clone());
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
        _ => {}
    }
}
