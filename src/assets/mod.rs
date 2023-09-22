use std::collections::HashMap;

use lazy_static::lazy_static;

// use crate::reference::read_file;

pub mod event;
pub mod technology;
pub mod building;

use crate::assets::{event::Modifier, technology::Technology};

lazy_static! {
    pub static ref EVENTS: HashMap<String, Vec<Modifier>> = event::parse_all_event();
    pub static ref TECHNOLOGIES: HashMap<String, Technology> = technology::parse_all_technology();
}

pub fn parse_all() {
    println!("初始化事件: {}", EVENTS.len());
    println!("初始化科技: {}", TECHNOLOGIES.len());
    building::parse_all_building();
}
