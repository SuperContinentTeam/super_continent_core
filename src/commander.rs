use crate::{
    assets::tags,
    reference::{AXState, AxClient},
    ws::send_message,
};

pub async fn join_room(client: AxClient, s: AXState, name: &str) {
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

pub async fn ready(client: AxClient, s: AXState, status: &str) {
    let name = &client.lock().await.player;
    let status: i32 = status.parse().unwrap();
    if let Some(p) = s.lock().await.players.get_mut(name) {
        p.ready = status;
    }
}

pub async fn player_leave(client: AxClient, s: AXState) {
    let name = { &client.lock().await.player };
    let mut s = s.lock().await;

    s.remove_player(name);
}

pub async fn update_state(client: AxClient, s: AXState, status: &str) {
    let c = client.lock().await;
    let mut s = s.lock().await;

    let name = &c.player;

    if &s.admin == name {
        let mut not_ready = 0;
        for p in s.players.values() {
            if p.ready == 0 {
                not_ready += 1;
            }
        }

        if not_ready == 0 {
            s.status = status.parse().unwrap();
        } else {
            send_message(
                format!("There are {} people is not ready", not_ready),
                client.clone(),
            )
            .await;
        }
    } else {
        send_message("You are not admin".to_string(), client.clone()).await;
    }
}

pub async fn change_player_tech_rate(client: AxClient, s: AXState, a: &str, b: &str, c: &str) {
    let name = { &client.lock().await.player };
    let mut s = s.lock().await;

    let player = s.players.get_mut(name).unwrap();
    if !player.has_tag(tags::CHANGE_TECH_RATE) {
        let a: u8 = a.parse().unwrap();
        let b: u8 = b.parse().unwrap();
        let c: u8 = c.parse().unwrap();
        player.set_tech_point(a, b, c);
    }
}

pub async fn bypass_binary(options: &str, client: AxClient, s: AXState) {
    let cmd = options.split(";").collect::<Vec<&str>>();
    println!("{:?}", cmd);

    let client = client.clone();
    let s = s.clone();

    match cmd[0] {
        "0" => join_room(client, s, cmd[1]).await,
        "1" => ready(client, s, cmd[1]).await,
        "2" => player_leave(client, s).await,
        "3" => update_state(client, s, cmd[1]).await,
        "4" => change_player_tech_rate(client, s, cmd[1], cmd[2], cmd[3]).await,
        _ => {}
    }
}
