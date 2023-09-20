use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cst;

use super::Dumps;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub storage: i32,
    pub daily: i32,

    pub projects: HashMap<String, i32>,
    pub modifiers: Vec<String>, // pub base_daily: i32,
}

// pub fn fix_daily(r: &Resource) -> i32 {
//     let origin_daily: i32 = r.projects.values().sum();

//     origin_daily
// }

impl Resource {
    pub fn new(storage: i32, daily: i32) -> Self {
        let mut p = HashMap::new();
        p.insert(cst::BASE.to_string(), daily);
        p.insert(cst::BLOCK.to_string(), 0);

        Self {
            storage,
            daily,
            projects: p,
            modifiers: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        self.storage += self.daily;
        if self.storage < 0 {
            self.storage = 0
        }
    }

    pub fn update_daily(&mut self) {
        let mut v = 0;
        for daily in self.projects.values() {
            v += daily;
        }
        self.daily = v;
    }
}

impl Dumps for Resource {
    fn dumps(&self, _player: &str) -> Value {
        json!({
            "storage": self.storage,
            "daily": self.daily,
            "projects": self.projects,
            "modifiers": self.modifiers
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct StateResource {
    pub resource_map: HashMap<String, Resource>,
}

impl StateResource {
    pub fn new() -> Self {
        let mut x = HashMap::new();
        x.insert(cst::ENERGY.to_string(), Resource::new(1000, 10));
        x.insert(cst::MINERAL.to_string(), Resource::new(1000, 10));
        x.insert(cst::FOOD.to_string(), Resource::new(1000, 10));
        x.insert(cst::ALLOY.to_string(), Resource::new(500, 5));
        x.insert(cst::CUSTOMER.to_string(), Resource::new(500, 5));
        x.insert(cst::TECHNOLOGY.to_string(), Resource::new(300, 3));

        Self { resource_map: x }
    }

    pub fn next(&mut self) {
        for res in self.resource_map.values_mut() {
            res.next();
        }
    }

    pub fn update_daily(&mut self) {
        for res in self.resource_map.values_mut() {
            res.update_daily();
        }
    }

    // pub fn update_resource_daily(&mut self, typ: &str, value: i32) {
    //     if let Some(res) = self.resource_map.get_mut(typ) {
    //         res.daily = value;
    //     }
    // }

    pub fn add_block_product(&mut self, typ: &str, value: i32) {
        let res = self.resource_map.get_mut(typ).unwrap();
        let entry = res.projects.entry(cst::BLOCK.to_string()).or_insert(0);
        *entry += value;
    }
}

impl Dumps for StateResource {
    fn dumps(&self, player: &str) -> Value {
        let v: HashMap<String, Value> = self
            .resource_map
            .iter()
            .map(|(typ, res)| (typ.to_string(), res.dumps(player)))
            .collect();

        json!(v)
    }
}
