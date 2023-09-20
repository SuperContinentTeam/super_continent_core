use std::collections::HashMap;

use crate::{reference::read_file, parse::to_tokens};

pub struct Modifier {
    pub code: String,
    pub resource: String,
    pub am: String,
}

impl Modifier {
    pub fn from_str(s: &str) -> Self {
        // "base:energy:add"
        let v = s.split(":").collect::<Vec<&str>>();
        assert!(v.len() == 3);
        Self {
            code: v[0].to_string(),
            resource: v[1].to_string(),
            am: v[2].to_string(),
        }
    }
}

pub struct Event {
    pub name: String,
    pub modifiers: Vec<Modifier>,
}

impl Event {
    pub fn new(name: &str, modifiers: Vec<Modifier>) -> Self {
        Self {
            name: name.to_string(),
            modifiers,
        }
    }
}

pub fn parse_event() -> HashMap<String, Event> {
    let mut result: HashMap<String, Event> = HashMap::new();
    let content = read_file("data/common/event.txt");

    let mut stack: Vec<String> = Vec::new();

    for token in to_tokens(&content) {
        if &token == ";" {
            let mut x = stack.pop();
            if let Some(c) = stack.pop() {

            }
        }
    }

    result
}

