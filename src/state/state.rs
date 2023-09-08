use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::state::resource::StateResource;

#[derive(Deserialize, Serialize)]
pub struct State {
    pub tick: u64,
    pub name: String,
    pub state_resource: StateResource,
}


impl State {
    pub fn new(name: String) -> Self {
        State {
            tick: 0,
            name,
            state_resource: StateResource::new(),
        }
    }

    pub fn next(&mut self) {
        self.tick += 1;
    }
}