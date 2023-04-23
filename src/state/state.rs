use std::sync::RwLock;
use crate::state::tick::Tick;
use once_cell::sync::OnceCell;

static GAME_STATE: OnceCell<GameState> = OnceCell::new();


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

pub fn get_game_state() -> &'static GameState {
    let game_state = match GAME_STATE.get() {
        None => {
            let game_state = GameState::new();
            GAME_STATE.set(game_state).unwrap();
            GAME_STATE.get().unwrap()
        }
        Some(game_state) => game_state
    };

    game_state
}
