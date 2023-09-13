use crate::{state::resource::StateResource, ws};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

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
    pub state_resource: StateResource,
    pub players: Vec<String>,
    pub status: u8 // 0: pause, 1: running, 2: exit
}

impl State {
    pub fn new(name: String, max_number: u8) -> Self {
        State {
            tick: 0,
            name,
            max_number,
            state_resource: StateResource::default(),
            players: Vec::new(),
            status: 0
        }
    }

    pub async fn next(&mut self) {
        self.tick += 1;
        self.state_resource.next();
        println!("State: {}, Tick: {}", self.name, self.tick);
    }

    pub fn can_join(&self, player: String) -> u8 {
        let use_number = self.players.len() as u8;
        if use_number < self.max_number {
            return 1;
        }

        if self.players.contains(&player) {
            return 2;
        }

        0
    }

    pub fn dumps(&self) -> String {
        let results = vec![
            self.name.clone(),
            self.tick.to_string(),
            self.max_number.to_string(),
            self.players.join(":"),
            self.state_resource.dumps(),
        ];

        results.join(";")
    }

    pub fn remove_player(&mut self, player: String) {
        self.players.retain(|x| x != &player);
    }
}

// 运行状态机
pub async fn run_state(state: AXState) {
    let state_clone = state.clone();
    let time_flow = TIME_FLOW.clone();

    loop {
        let mut s = state_clone.lock().await;
        if s.status == 1 {
            s.next().await;
            ws::broadcast(&s.players, s.dumps()).await;
            // ws::broadcast(&s.players, &s.to_json()).await;
        } else if s.status == 2 {
            break;
        }
        tokio::time::sleep(time_flow).await;
    }
}
