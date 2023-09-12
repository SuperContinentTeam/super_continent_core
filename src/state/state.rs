use crate::state::resource::StateResource;
use lazy_static::lazy_static;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

pub type AXState = Arc<Mutex<State>>;
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
    pub pause: bool
}

impl State {
    pub fn new(name: String, max_number: u8) -> Self {
        State {
            tick: 0,
            name,
            max_number,
            state_resource: StateResource::default(),
            players: Vec::new(),
            pause: true
        }
    }

    pub fn next(&mut self) {
        self.tick += 1;
        println!("State: {}, Tick: {}", self.name, self.tick);
    }

    pub fn can_join(&self) -> bool {
        let use_number = self.players.len() as u8;
        use_number < self.max_number
    }
}

// 运行状态机
pub async fn run_state(state: AXState) {
    let state_clone = state.clone();
    let time_flow = TIME_FLOW.clone();

    loop {
        let mut s = state_clone.lock().await;
        if !s.pause {
            s.next();
        }
        tokio::time::sleep(time_flow).await;
    }
}
