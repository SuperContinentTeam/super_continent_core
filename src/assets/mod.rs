use std::collections::HashMap;

use lazy_static::lazy_static;

// use crate::reference::read_file;

use self::event::Event;

pub mod event;

lazy_static! {
    pub static ref EVENT_MAP: HashMap<String, Event> = HashMap::new();
}



pub fn parse_all() {
    event::parse_event();
}