use crate::{state::resource::StateResource, ws};
use lazy_static::lazy_static;
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

    pub async fn next(&mut self) {
        self.tick += 1;
        self.state_resource.next();
        println!("State: {}, Tick: {}", self.name, self.tick);
        ws::broadcast(&self.players, &self.to_json()).await;
    }

    pub fn can_join(&self) -> bool {
        let use_number = self.players.len() as u8;
        use_number < self.max_number
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tick": self.tick,
            "name": self.name,
            "max_number": self.max_number,
            "players": self.players,
            "resources": self.state_resource
        })
    }
}

// 运行状态机
pub async fn run_state(state: AXState) {
    let state_clone = state.clone();
    let time_flow = TIME_FLOW.clone();

    loop {
        let mut s = state_clone.lock().await;
        if !s.pause {
            s.next().await;
        }
        tokio::time::sleep(time_flow).await;
    }
}
