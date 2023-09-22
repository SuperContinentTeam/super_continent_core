use std::collections::HashMap;
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::cst;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub tier: u8,
    pub tick: u64,
    pub zoning_need: u8,
    pub front_techs: Option<Vec<String>>,
    pub front_buildings: Option<Vec<String>>,
    pub materials: HashMap<String, u64>,
}

fn parse_building(area: &str) {
    let content = read_to_string(&format!("data/common/buildings/{}.yaml", area)).unwrap();
    let v: HashMap<String, Building> = serde_yaml::from_str(&content).unwrap();
}

pub fn parse_all_building() {
    parse_building(cst::BLOCK);
}
