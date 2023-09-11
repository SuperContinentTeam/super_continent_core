use crate::state::resource::StateResource;
use lazy_static::lazy_static;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub type AXState = Arc<Mutex<State>>;
type StateMap = Arc<Mutex<HashMap<String, AXState>>>;

lazy_static! {
    pub static ref STATE_MAP: StateMap = StateMap::default();
    pub static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
}

#[derive(Deserialize, Serialize)]
pub struct State {
    pub tick: u64,
    pub name: String,
    pub use_number: u8,
    pub max_number: u8,
    pub state_resource: StateResource,
    pub players: Vec<String>
}

impl State {
    pub fn new(name: String, max_number: u8) -> Self {
        State {
            tick: 0,
            name,
            use_number: 1,
            max_number,
            state_resource: StateResource::default(),
            players: Vec::default(),
        }
    }

    pub fn next(&mut self) {
        self.tick += 1;
        println!("State: {}, Tick: {}", self.name, self.tick);
    }
}

// 检查状态机
pub async fn check_state(room: &str) -> Option<AXState> {
    let state_map = STATE_MAP.clone();
    let state_map = state_map.lock().await;
    let state = state_map.get(room);

    match state {
        Some(state) => {
            let s = state.lock().await;
            let result = {
                if s.use_number < s.max_number {
                    Some(state.clone())
                } else {
                    None
                }
            };
            result
        }
        None => None,
    }
}

// 新增状态机
pub async fn add_state(value: Value) {
    let name = value.get("name").unwrap().to_string();
    let max_number = value.get("maxNumber").unwrap().as_u64().unwrap();

    let state_map_clone = STATE_MAP.clone();
    let mut state_map = state_map_clone.lock().await;

    let state = Arc::new(Mutex::new(State::new(name.clone(), max_number as u8)));

    state_map.insert(name.clone(), state.clone());
    tokio::task::spawn(run_state(state.clone()));
}

// 运行状态机
async fn run_state(state: AXState) {
    let state_clone = state.clone();
    let time_flow = TIME_FLOW.clone();

    loop {
        let mut s = state_clone.lock().await;
        s.next();

        tokio::time::sleep(time_flow).await;
    }
}
