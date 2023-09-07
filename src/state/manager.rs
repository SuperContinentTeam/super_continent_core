use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::state::state::State;

type AXState = Arc<Mutex<State>>;

pub struct StateManager {
    pub hall: HashMap<String, AXState>,
}


impl StateManager {
    pub fn new() -> Self {
        let mut h: HashMap<String, AXState> = HashMap::new();
        let x = Arc::new(Mutex::new(State::new("A".to_string())));
        h.insert("A".to_string(), x);

        let y = Arc::new(Mutex::new(State::new("B".to_string())));
        h.insert("B".to_string(), y);

        StateManager {
            hall: h
        }
    }

    pub async fn next(&mut self) {
        for (_, state) in self.hall.iter_mut() {
            let mut state_clone = state.clone();
            let join = tokio::task::spawn_blocking(move || {
                let mut ms = state_clone.lock().unwrap();
                ms.next();
            });
            let _ = join.await.unwrap();
        }
    }
}