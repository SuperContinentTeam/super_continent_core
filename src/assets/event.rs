use std::collections::HashMap;

use crate::{asset_parse::to_tokens, cst, reference::read_file};

pub type Events = Vec<String>;
#[derive(Debug)]
pub struct Modifier {
    pub code: String,
    pub entity: String,
    pub method: String,
    pub value: f64,
}

fn parse_event(area: &str) -> HashMap<String, Vec<Modifier>> {
    let content = read_file(&format!("data/common/events/{}.txt", area));
    let mut stack: Vec<String> = Vec::new();
    let mut temp_modifier: HashMap<String, String> = HashMap::new();
    let mut result: HashMap<String, Vec<Modifier>> = HashMap::new();

    for i in to_tokens(&content) {
        match i.as_str() {
            "}" => {
                let name = stack.pop().unwrap();
                let mut temp: Vec<Modifier> = Vec::new();

                for (code, value) in &temp_modifier {
                    let cs: Vec<&str> = code.split(":").collect();
                    let modifier = Modifier {
                        code: area.to_string(),
                        entity: cs[0].to_string(),
                        method: cs[1].to_string(),
                        value: value.parse::<f64>().unwrap(),
                    };
                    temp.push(modifier);
                }
                
                result.insert(name, temp);
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

    result
}

pub fn parse_all_event() -> HashMap<String, Vec<Modifier>> {
    let mut result: HashMap<String, Vec<Modifier>> = HashMap::new();

    result.extend(parse_event(cst::BLOCK));

    result
}
