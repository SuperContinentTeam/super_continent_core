use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub tick: usize,
    pub name: String,
}

impl State {
    pub fn new(name: String) -> State {
        State {
            tick: 1,
            name,
        }
    }

    pub fn next(&mut self) -> Result<(), Box<dyn Error>> {
        self.tick += 1;

        Ok(())
    }
}