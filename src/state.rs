use serde_json::{json, Value};

use crate::cst;
use crate::game::Dumps;
use crate::reference::{AXState, AxClient, TIME_FLOW};
use std::collections::HashMap;

use crate::ws::send_message;

use crate::{game::world::World, player::Player};

pub struct State {
    pub tick: u64,
    pub players: HashMap<String, Player>,
    pub admin: String,
    pub max_player: i32,
    pub status: i32, // 0: pause, 1: running, 2: exit
    pub world: World,
}

impl State {
    pub async fn next(&mut self) {
        self.tick += 1;
        for (_, player) in self.players.iter_mut() {
            player.next();
        }
        self.world.next();

        println!("State Tick: {}", self.tick);
    }

    pub fn add_player(&mut self, name: &str, client: AxClient) {
        let mut player = Player::new(client, name.to_string());

        let b = self.world.blocks.get_mut(&self.world.rand_block()).unwrap();
        b.initial_people();

        player.add_block(b);
        player.build_solider(b);
        
        if self.players.len() == 0 {
            self.admin = name.to_string();
        }

        self.players.insert(name.to_string(), player);
    }

    pub fn remove_player(&mut self, player: &str) {
        if let Some(p) = self.players.remove(player) {
            for pos in p.blocks {
                self.world.blocks.get_mut(&pos).unwrap().set_player(None);
            }
        }

        if self.players.len() == 0 {
            self.status = 0;
        }
    }

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

    pub async fn broadcast(&self) {
        for player in self.players.values() {
            let message = self.dumps(&player.name);
            tokio::task::spawn(send_message(message.to_string(), player.client.clone()));
        }
    }

    // pub async fn broadcast_message(&self, message: &str) {
    //     for player in self.players.values() {
    //         tokio::task::spawn(send_message(message.to_string(), player.client.clone()));
    //     }
    // }
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

impl Dumps for State {
    fn dumps(&self, player: &str) -> Value {
        let p = self.players.get(player).unwrap();

        json!({
            cst::DUMP_TICK: self.tick,
            cst::DUMP_WORLD: self.world.dumps(player),
            cst::DUMP_PLAYER: p.dumps(player)
        })
    }
}
