use std::collections::HashMap;

use crate::{asset_parse::to_tokens, cst, reference::read_file};

#[derive(Debug)]
pub struct Technology {
    pub area: String,
    pub tier: u8,
    pub cost: u64,
    pub fronts: Vec<String>,
}

fn parse_area_technology(area: &str) -> HashMap<String, Technology> {
    let content = read_file(&format!("data/common/technologies/{}.txt", area));
    let mut result: HashMap<String, Technology> = HashMap::new();

    let mut fronts: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut is_front = false;
    let mut tier: u8 = 0;
    let mut cost: u64 = 0;

    for i in to_tokens(&content) {
        match i.as_str() {
            "}" => {
                if is_front {
                    is_front = false;
                    stack.pop();
                } else {
                    let name = stack.first().unwrap();
                    let t = Technology {
                        area: area.to_string(),
                        tier,
                        cost,
                        fronts: fronts.clone(),
                    };
                    result.insert(name.to_string(), t);
                    fronts.clear();
                    stack.clear();
                }
            }
            ";" => {
                let mut temp = String::new();
                let mut x = stack.pop().unwrap();

                while &x != "{" {
                    if &x == "tier" {
                        tier = temp.parse().unwrap();
                    } else if &x == "cost" {
                        cost = temp.parse().unwrap();
                    } else {
                        if &x != "=" {
                            temp = x;
                        }
                        if is_front {
                            fronts.push(temp.clone());
                        }
                    }

                    x = stack.pop().unwrap();
                }
                stack.push("{".to_string());
            }
            "fronts" => is_front = true,
            "=" => {}
            _ => {
                stack.push(i);
            }
        }
    }

    result
}

pub fn parse_all_technology() -> HashMap<String, Technology> {
    let mut result: HashMap<String, Technology> = HashMap::new();
    result.extend(parse_area_technology(cst::CIVILIAN));

    result
}
