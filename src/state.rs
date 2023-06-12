use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref STATE_MAP: Arc<Mutex<HashMap<String, Arc<Mutex<GameState>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct GameState {
    pub name: String,
    pub tick: u32,
}

pub fn initial() {
    let _ = STATE_MAP.clone().lock().unwrap().insert(
        "localhost".to_string(),
        Arc::new(Mutex::new(GameState::new("localhost".to_string()))),
    );
}

impl GameState {
    pub fn new(name: String) -> Self {
        Self { name, tick: 0 }
    }
}
