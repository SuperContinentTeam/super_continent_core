use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::ws::{send_message, get_clients};

use super::player::Player;

pub type AXState = Arc<Mutex<State>>;
// RoomName -> State
type StateMap = Arc<Mutex<HashMap<String, AXState>>>;

lazy_static! {
    pub static ref STATE_MAP: StateMap = StateMap::default();
    pub static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
}

pub struct State {
    pub tick: u64,
    pub name: String,
    pub max_number: u8,
    pub players: HashMap<String, Player>,
    pub status: u8, // 0: pause, 1: running, 2: exit
}

impl State {
    pub fn new(name: String, max_number: u8) -> Self {
        State {
            tick: 0,
            name,
            max_number,
            players: HashMap::new(),
            status: 0,
        }
    }

    pub async fn next(&mut self) {
        self.tick += 1;
        for (_ , player) in self.players.iter_mut() {
            player.next();
        }
        println!("State: {}, Tick: {}", self.name, self.tick);
    }

    pub fn add_player(&mut self, name: &str) {
        self.players.insert(name.to_string(), Player::new(name.to_string()));
    }

    pub fn remove_player(&mut self, player: String) {
        self.players.remove(&player);
    }

    pub fn can_join(&self, player: &str) -> u8 {
        let use_number = self.players.len() as u8;
        if use_number >= self.max_number {
            return 1;
        }

        if self.players.contains_key(player) {
            return 2;
        }

        0
    }

    pub fn dump_by_one(&self, name: &String) -> String {
        let player = self.players.get(name).unwrap();
        let results = vec![
            self.tick.to_string(),
            player.dumps()
        ];

        results.join(";")
    }



    pub async fn broadcast(&self) {
        let clients= get_clients(self.players.keys()).await;
        for (name, _) in self.players.iter() {
            let message = self.dump_by_one(name);
            if let Some(c) = clients.get(name) {
                tokio::task::spawn(send_message(message, c.clone()));
            }
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
