use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cst;

use super::Dumps;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub typ: String,
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
    pub fn new(typ: &str, storage: i32, daily: i32) -> Self {
        let mut p = HashMap::new();
        p.insert("base".to_string(), daily);
        p.insert(cst::BLOCK.to_string(), 0);

        Self {
            typ: typ.to_string(),
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
    pub energy: Resource,
    pub mineral: Resource,
    pub food: Resource,
    pub customer: Resource,
    pub alloy: Resource,
    pub technology: Resource,
}

impl StateResource {
    pub fn new() -> Self {
        Self {
            energy: Resource::new("e", 1000, 10),
            mineral: Resource::new("m", 1000, 10),
            food: Resource::new("f", 1000, 10),
            customer: Resource::new("c", 500, 5),
            alloy: Resource::new("a", 500, 5),
            technology: Resource::new("t", 300, 3),
        }
    }

    pub fn next(&mut self) {
        self.energy.next();
        self.mineral.next();
        self.food.next();
        self.customer.next();
        self.alloy.next();
        self.technology.next();
    }

    pub fn update_daily(&mut self) {
        self.energy.update_daily();
        self.mineral.update_daily();
        self.food.update_daily();
        self.customer.update_daily();
        self.alloy.update_daily();
        self.technology.update_daily();
    }
}

impl Dumps for StateResource {
    fn dumps(&self, player: &str) -> Value {
        json!({
            cst::ENERGY: self.energy.dumps(player),
            cst::MINERAL: self.mineral.dumps(player),
            cst::FOOD: self.food.dumps(player),
            cst::CUSTOMER: self.customer.dumps(player),
            cst::ALLOY: self.alloy.dumps(player),
            cst::TECHNOLOGY: self.technology.dumps(player)
        })
    }
}
