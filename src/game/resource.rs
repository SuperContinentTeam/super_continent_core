use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cst;

use super::Dumps;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub storage: f64,
    pub daily: f64,

    // pub projects: HashMap<String, i32>,
}

// pub fn fix_daily(r: &Resource) -> i32 {
//     let origin_daily: i32 = r.projects.values().sum();

//     origin_daily
// }

impl Resource {
    pub fn new(storage: f64, daily: f64) -> Self {
        // let mut p = HashMap::new();
        // 基础资源产出
        // p.insert(cst::BASE.to_string(), daily);
        // 地块资源产出
        // p.insert(cst::BLOCK.to_string(), 0);
        Self {
            storage,
            daily,
            // projects: p,
        }
    }

    pub fn next(&mut self) {
        self.storage += self.daily;
        if self.storage < 0.0 {
            self.storage = 0.0
        }
    }
}

impl Dumps for Resource {
    fn dumps(&self, _player: &str) -> Value {
        json!({
            "storage": self.storage,
            "daily": self.daily,
            // "projects": self.projects
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
        x.insert(cst::ENERGY.to_string(), Resource::new(1000.0, 10.0));
        x.insert(cst::MINERAL.to_string(), Resource::new(1000.0, 10.0));
        x.insert(cst::FOOD.to_string(), Resource::new(1000.0, 10.0));
        x.insert(cst::ALLOY.to_string(), Resource::new(500.0, 5.0));
        x.insert(cst::CUSTOMER.to_string(), Resource::new(500.0, 5.0));
        x.insert(cst::TECHNOLOGY.to_string(), Resource::new(300.0, 3.0));

        Self { resource_map: x }
    }

    pub fn next(&mut self) {
        for res in self.resource_map.values_mut() {
            res.next();
        }
    }

    // pub fn update_daily(&mut self) {
    //     for res in self.resource_map.values_mut() {
    //         res.update_daily();
    //     }
    // }

    // pub fn update_resource_daily(&mut self, typ: &str, value: i32) {
    //     if let Some(res) = self.resource_map.get_mut(typ) {
    //         res.daily = value;
    //     }
    // }

    pub fn update_daily_with_block(&mut self, product: (f64, f64, f64)) {
        let rs = vec![
            (cst::ENERGY, product.0), (cst::MINERAL, product.1), (cst::FOOD, product.2)
        ];

        for (res, value) in rs {
            let res = self.resource_map.get_mut(res).unwrap();

            // let entry = res.projects.entry(cst::BLOCK.to_string()).or_insert(0);
            // *entry += value;
            res.daily += value;
        }
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
