use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::state::state::State;
use crate::state::NextState;

type AXState = Arc<Mutex<State>>;

lazy_static! {
    static ref STATE_MAP: Arc<Mutex<HashMap<String, AXState>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
}


// fn run_state(s: Arc<Mutex<State>>) {
//     let time_flow_clone = TIME_FLOW.clone();
//     loop {
//         let s_clone = s.clone();
//         let mut ax_s = s_clone.lock().unwrap();
//         ax_s.next();
//         println!("State: {}, Tick: {}", ax_s.name, ax_s.tick);
//         std::thread::sleep(time_flow_clone);
//     }
// }

// pub fn add_state(value: serde_json::Value) {
//     let name = value.get("name").unwrap().to_string();

//     let state_map_clone = STATE_MAP.clone();
//     let mut state_map = state_map_clone.lock().unwrap();

//     let state = Arc::new(Mutex::new(State::new(name.clone())));
//     state_map.insert(name.clone(), state.clone());

//     // 创建一个子线程 单独运行该状态机
//     let state_clone = state.clone();

//     let t = std::thread::spawn(move || {
//         run_state(state_clone);
//     });
//     t.join().unwrap();
// }
