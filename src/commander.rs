use crate::{
    reference::{AXState, AxClient},
    ws::send_message,
};

pub async fn join_room(name: &str, client: AxClient, s: AXState) {
    println!("Player join the room: {}", name);
    let mut s = s.lock().await;

    let can_status = s.can_join(name);
    if can_status != 1 {
        send_message(can_status.to_string(), client.clone()).await;
        return;
    }

    s.add_player(name, client.clone());
    client.lock().await.player = name.to_string();
}

pub async fn ready(status: &str, client: AxClient, s: AXState) {
    let c = client.lock().await;
    let mut s = s.lock().await;

    let status: i32 = status.parse().unwrap();
    s.player_ready(&c.player, status).await;
}

pub async fn player_leave(client: AxClient, s: AXState) {
    let c = client.lock().await;
    let mut s = s.lock().await;

    s.remove_player(&c.player);
}

pub async fn bypass_binary(options: &str, client: AxClient, s: AXState) {
    let cmd = options.split(";").collect::<Vec<&str>>();
    println!("{:?}", cmd);

    let client = client.clone();
    let s = s.clone();

    match cmd[0] {
        "0" => join_room(cmd[1], client, s).await,
        "1" => ready(cmd[1], client, s).await,
        "2" => player_leave(client, s).await,
        _ => {}
    }
}
