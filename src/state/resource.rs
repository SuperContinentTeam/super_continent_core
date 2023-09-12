use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub typ: String,
    pub storage: i64,
    pub daily: i64,
}

impl Resource {
    pub fn next(&mut self) {
        self.storage += self.daily
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
    pub fn next(&mut self) {
        self.energy.next();
        self.mineral.next();
        self.food.next();
        self.customer.next();
        self.alloy.next();
        self.technology.next();
    }
}

impl Default for StateResource {
    fn default() -> Self {
        Self {
            energy: Resource {
                typ: "energy".to_string(),
                storage: 1000,
                daily: 10,
            },
            mineral: Resource {
                typ: "mineral".to_string(),
                storage: 1000,
                daily: 10,
            },
            food: Resource {
                typ: "food".to_string(),
                storage: 1000,
                daily: 10,
            },
            customer: Resource {
                typ: "customer".to_string(),
                storage: 500,
                daily: 5,
            },
            alloy: Resource {
                typ: "alloy".to_string(),
                storage: 500,
                daily: 5,
            },
            technology: Resource {
                typ: "technology".to_string(),
                storage: 300,
                daily: 3,
            },
        }
    }
}