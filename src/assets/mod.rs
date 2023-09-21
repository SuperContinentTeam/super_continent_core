use std::collections::HashMap;

use lazy_static::lazy_static;

use self::event::Modifier;

// use crate::reference::read_file;

pub mod event;

lazy_static! {
    pub static ref BLOCK_EVENTS: HashMap<String, Vec<Modifier>> = event::parse_event("block");
}



pub fn parse_all() {
    println!("初始化事件数: {}", BLOCK_EVENTS.len());
    let v = BLOCK_EVENTS.get("env_n2").unwrap();
    println!("{:#?}", v);
}