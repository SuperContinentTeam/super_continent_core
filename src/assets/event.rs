use std::collections::HashMap;

use crate::{asset_parse::to_tokens, reference::read_file};

#[derive(Debug)]
pub struct Modifier {
    pub code: String,
    pub resource: String,
    pub method: String,
    pub value: f64,
}

pub fn parse_event() -> HashMap<String, Vec<Modifier>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let content = read_file("data/common/events.txt");
    let mut stack: Vec<String> = Vec::new();
    let mut temp_modifier: HashMap<String, String> = HashMap::new();

    for i in to_tokens(&content) {
        match i.as_str() {
            "}" => {
                let name = stack.pop().unwrap();
                result.insert(name, temp_modifier.clone());
                temp_modifier.clear();
            }
            ";" => {
                let temp_value = stack.pop().unwrap();
                let temp_code = stack.pop().unwrap();
                temp_modifier.insert(temp_code, temp_value);
            }
            "{" | "=" => {}
            _ => {
                stack.push(i);
            }
        }
    }

    let mut real_result: HashMap<String, Vec<Modifier>> = HashMap::new();
    for (event_name, modifier_map) in result {
        let mut temp: Vec<Modifier> = Vec::new();
        for (code, value) in modifier_map {
            let cs: Vec<&str> = code.split(":").collect();
            let modifier = Modifier {
                code: cs[0].to_string(),
                resource: cs[1].to_string(),
                method: cs[2].to_string(),
                value: value.parse::<f64>().unwrap(),
            };
            temp.push(modifier);
        }

        real_result.insert(event_name, temp);
    }

    real_result
}
