use crate::state::tick::Tick;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

static GAME_STATE_MAP: Lazy<Mutex<HashMap<String, Arc<GameState>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug)]
pub struct GameState {
    pub pause: RwLock<bool>,
    pub tick: RwLock<Tick>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            pause: RwLock::new(true),
            tick: RwLock::new(Tick::new()),
        }
    }

    pub fn time_flow(&self) {
        let mut tick = self.tick.write().unwrap();
        tick.value += 1;
    }
}

pub fn get_game_state(name: String) -> Arc<GameState> {
    let mut map = GAME_STATE_MAP.lock().unwrap();

    match map.get(name.as_str()) {
        None => {
            let gs = GameState::new();
            let arc_gs = Arc::new(gs);
            map.insert(name, arc_gs.clone());
            arc_gs.clone()
        }
        Some(gs) => gs.clone(),
    }
}
