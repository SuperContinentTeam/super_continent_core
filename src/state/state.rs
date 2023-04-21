use std::sync::RwLock;

use crate::state::tick::Tick;


#[derive(Debug)]
pub struct GameState{
    pub pause: RwLock<bool>,    
    pub tick: RwLock<Tick>
}

impl GameState{
    pub fn new() -> Self {
        Self {
            pause: RwLock::new(true),
            tick: RwLock::new(Tick::new())
        }
    }

    pub fn time_flow(&self) {
        let mut tick = self.tick.write().unwrap();
        tick.value += 1;
    }
}