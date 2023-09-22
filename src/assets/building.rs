use std::collections::HashMap;
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::cst;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub area: String,
    pub tier: u8,
    pub tick: u64,
    pub zoning_need: u8,
    pub front_techs: Option<Vec<String>>,
    pub front_buildings: Option<Vec<String>>,
    pub materials: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Temp {
    pub tier: u8,
    pub tick: u64,
    pub zoning_need: u8,
    pub front_techs: Option<Vec<String>>,
    pub front_buildings: Option<Vec<String>>,
    pub materials: HashMap<String, u64>,
}

impl Building {
    fn from_temp(area: &str, temp: Temp) -> Self {
        Self {
            area: area.to_string(),
            tier: temp.tier,
            tick: temp.tick,
            zoning_need: temp.zoning_need,
            front_techs: temp.front_techs,
            front_buildings: temp.front_buildings,
            materials: temp.materials,
        }
    }
}

fn parse_building(area: &str) -> HashMap<String, Building> {
    let content = read_to_string(&format!("data/common/buildings/{}.yaml", area)).unwrap();
    let mut v: HashMap<String, Temp> = serde_yaml::from_str(&content).unwrap();

    v.drain()
        .map(|(name, temp)| {
            let building = Building::from_temp(area, temp);
            (name, building)
        })
        .collect::<HashMap<String, Building>>()
}

pub fn parse_all_building() -> HashMap<String, Building> {
    let mut result: HashMap<String, Building> = HashMap::new();
    result.extend(parse_building(cst::BLOCK));

    // println!("{:#?}", result);
    result
}
