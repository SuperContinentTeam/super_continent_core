use std::collections::HashMap;

use lazy_static::lazy_static;

use self::event::Modifier;

// use crate::reference::read_file;

pub mod event;

lazy_static! {
    pub static ref EVENT_MAP: HashMap<String, Vec<Modifier>> = event::parse_event();
}



pub fn parse_all() {
    println!("初始化事件数: {}", EVENT_MAP.len());
}