use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub typ: String,
    pub storage: i64,
    pub daily: i64,
}

impl Resource {
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

impl Default for StateResource {
    fn default() -> Self {
        Self {
            energy: Resource {
                typ: "e".to_string(), // energy
                storage: 1000,
                daily: 10,
            },
            mineral: Resource {
                typ: "m".to_string(), // mineral
                storage: 1000,
                daily: 10,
            },
            food: Resource {
                typ: "f".to_string(), // food
                storage: 1000,
                daily: 10,
            },
            customer: Resource {
                typ: "c".to_string(), // customer
                storage: 500,
                daily: 5,
            },
            alloy: Resource {
                typ: "a".to_string(), // alloy
                storage: 500,
                daily: 5,
            },
            technology: Resource {
                typ: "t".to_string(), // technology
                storage: 300,
                daily: 3,
            },
        }
    }
}
