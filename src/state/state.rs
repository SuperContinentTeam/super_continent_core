use std::error::Error;

pub struct State {
    pub tick: u64,
    pub name: String,
}


impl State {
    pub fn new(name: String) -> Self {
        State {
            tick: 0,
            name,
        }
    }

    pub fn next(&mut self) {
        self.tick += 1;

        println!("State: {}, tick: {}", self.name, self.tick);
    }
}