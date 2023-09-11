use serde::{Deserialize, Serialize};
use crate::state::NextState;
use crate::state::resource::StateResource;


#[derive(Deserialize, Serialize)]
pub struct State {
    pub tick: u64,
    pub name: String,
    pub use_number: u8,
    pub max_number: u8,
    pub state_resource: StateResource,
}


impl State {
    pub fn new(name: String, max_number: u8) -> Self {
        State {
            tick: 0,
            name,
            use_number: 0,
            max_number,
            state_resource: StateResource::new(),
        }
    }
}

impl NextState for State {
    fn next(&mut self) {
        self.tick += 1;

        println!("State: {}, Tick: {}", self.name, self.tick);
    }
}