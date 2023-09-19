use crate::reference::{AXState, AxClient, TIME_FLOW};
use std::collections::HashMap;

use crate::ws::send_message;

use crate::{game::world::World, player::Player};

pub struct State {
    pub tick: u64,
    pub players: HashMap<String, Player>,
    pub max_player: i32,
    // 0: pause, 1: running, 2: exit
    pub status: i32,
    pub world: World,
}

impl State {
    pub async fn next(&mut self) {
        self.tick += 1;
        for (_, player) in self.players.iter_mut() {
            player.next();
        }
        println!("State Tick: {}", self.tick);
    }

    pub fn add_player(&mut self, name: &str, client: AxClient) {
        let mut player = Player::new(client, name.to_string());

        let pos = self.world.rand_block();
        let b = self.world.blocks.get_mut(&pos).unwrap();

        b.belong = Some(name.to_string());
        player.blocks.push(pos);

        self.players.insert(name.to_string(), player);
    }

    // pub fn remove_player(&mut self, player: String) {
    //     if let Some(p) = self.players.remove(&player) {
    //         for pos in p.blocks {
    //             let b = self.world.blocks.get_mut(&pos).unwrap();
    //             b.belong = None;
    //         }
    //     }
    // }

    pub fn can_join(&self, player: &str) -> i32 {
        let use_number = self.players.len() as i32;
        if use_number >= self.max_player {
            return 0;
        }

        if self.players.contains_key(player) {
            return 2;
        }

        1
    }

    pub fn dump_by_one(&self, name: &String) -> String {
        let player = self.players.get(name).unwrap();
        let results = vec![self.tick.to_string(), player.dumps()];

        results.join(";")
    }

    pub async fn broadcast(&self) {
        for player in self.players.values() {
            let message = self.dump_by_one(&player.name);
            tokio::task::spawn(send_message(message, player.client.clone()));
        }
    }

    pub async fn broadcast_message(&self, message: &str) {
        for player in self.players.values() {
            tokio::task::spawn(send_message(message.to_string(), player.client.clone()));
        }
    }

    pub async fn player_ready(&mut self, name: &str, status: i32) {
        if let Some(p) = self.players.get_mut(name) {
            p.ready = status;
        }

        let mut can_start = true;
        for p in self.players.values() {
            if p.ready == 0 {
                can_start = false;
                break;
            }
        }

        if can_start {
            self.status = 1;
            // 广播消息, 通知客户的可以开始
            self.broadcast_message("01").await;
        }
    }
}

// 运行状态机
pub async fn run_state(state: AXState) {
    let state_clone = state.clone();
    let time_flow = TIME_FLOW.clone();

    loop {
        let mut s = state_clone.lock().await;
        if s.status == 1 {
            s.broadcast().await;
            s.next().await;
        } else if s.status == 2 {
            break;
        }
        tokio::time::sleep(time_flow).await;
    }
}
