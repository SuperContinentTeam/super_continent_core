use std::{collections::HashMap, fs::read_to_string};

use serde::{Deserialize, Serialize};

use crate::cst;

#[derive(Debug, Serialize, Deserialize)]
pub struct Technology {
    pub area: String,
    pub tier: u8,
    pub cost: u64,
    pub fronts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    pub tier: u8,
    pub cost: u64,
    pub fronts: Option<Vec<String>>,
}

fn parse_area_technology(area: &str) -> HashMap<String, Technology> {
    let content = read_to_string(&format!("data/common/technologies/{}.yaml", area)).unwrap();
    let mut v: HashMap<String, Temp> = serde_yaml::from_str(&content).unwrap();

    v.drain()
        .map(|(name, mut temp)| {
            let fronts = std::mem::take(&mut temp.fronts);
            let t = Technology {
                area: area.to_string(),
                tier: temp.tier,
                cost: temp.cost,
                fronts,
            };
            (name.clone(), t)
        })
        .collect::<HashMap<String, Technology>>()
}

pub fn parse_all_technology() -> HashMap<String, Technology> {
    let mut result: HashMap<String, Technology> = HashMap::new();
    result.extend(parse_area_technology(cst::CIVILIAN));

    // println!("{:#?}", result);
    result
}
