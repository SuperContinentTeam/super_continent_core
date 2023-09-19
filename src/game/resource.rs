use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

    pub fn dumps(&self) -> String {
        format!("{},{},{}", self.typ, self.storage, self.daily)
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

    pub fn dumps(&self) -> String {
        let results = vec![
            self.energy.dumps(),
            self.mineral.dumps(),
            self.food.dumps(),
            self.customer.dumps(),
            self.alloy.dumps(),
            self.technology.dumps(),
        ];

        results.join(":")
    }
}