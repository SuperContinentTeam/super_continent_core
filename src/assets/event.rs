use std::{collections::HashMap, fs::read_to_string};

use crate::cst;

pub type Events = Vec<String>;
#[derive(Debug)]
pub struct Modifier {
    pub code: String,
    pub entity: String,
    pub method: String,
    pub value: f64,
}

fn parse_event(area: &str) -> HashMap<String, Vec<Modifier>> {
    let content = read_to_string(&format!("data/common/events/{}.yaml", area)).unwrap();
    let mut v: HashMap<String, HashMap<String, f64>> = serde_yaml::from_str(&content).unwrap();

    v.drain()
        .map(|(event_name, modifiers)| {
            let ms = modifiers
                .iter()
                .map(|(code, value)| {
                    let c: Vec<&str> = code.split("_").collect();
                    Modifier {
                        code: area.to_string(),
                        entity: c[0].to_string(),
                        method: c[1].to_string(),
                        value: *value,
                    }
                })
                .collect::<Vec<Modifier>>();

            (event_name, ms)
        })
        .collect::<HashMap<String, Vec<Modifier>>>()
}

pub fn parse_all_event() -> HashMap<String, Vec<Modifier>> {
    let mut result: HashMap<String, Vec<Modifier>> = HashMap::new();
    result.extend(parse_event(cst::BLOCK));

    // println!("{:#?}", result);
    result
}
