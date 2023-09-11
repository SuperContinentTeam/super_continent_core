use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::state::state::State;
use crate::state::NextState;

type AXState = Arc<Mutex<State>>;
type StateMap = Arc<Mutex<HashMap<String, AXState>>>;

lazy_static! {
    pub static ref STATE_MAP: Arc<Mutex<HashMap<String, AXState>>> = StateMap::default();
    static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
}

pub fn add_state(value: serde_json::Value) {
    let name = value.get("name").unwrap().to_string();
    let max_number = value.get("maxNumber").unwrap().as_u64().unwrap();

    let state_map_clone = STATE_MAP.clone();
    let mut state_map = state_map_clone.lock().unwrap();

    let state = Arc::new(Mutex::new(State::new(name.clone(), max_number as u8)));

    state_map.insert(name.clone(), state.clone());

    let state_clone = state.clone();
    let time_flow_clone = TIME_FLOW.clone();
    std::thread::spawn(move || loop {
        let s_clone = state_clone.clone();
        let mut s = s_clone.lock().unwrap();
        s.next();

        std::thread::sleep(time_flow_clone);
    });
}
