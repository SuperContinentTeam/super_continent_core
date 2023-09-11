use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ResourceType {
    ENERGY,
    MINERAL,
    FOOD,
    CUSTOMER,
    ALLOY,
    TECHNOLOGY,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub typ: ResourceType,
    pub storage: i64,
    pub daily: i64,
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

impl Default for StateResource {
    fn default() -> Self {
        Self {
            energy: Resource {
                typ: ResourceType::ENERGY,
                storage: 1000,
                daily: 10,
            },
            mineral: Resource {
                typ: ResourceType::MINERAL,
                storage: 1000,
                daily: 10,
            },
            food: Resource {
                typ: ResourceType::FOOD,
                storage: 1000,
                daily: 10,
            },
            customer: Resource {
                typ: ResourceType::CUSTOMER,
                storage: 500,
                daily: 5,
            },
            alloy: Resource {
                typ: ResourceType::ALLOY,
                storage: 500,
                daily: 5,
            },
            technology: Resource {
                typ: ResourceType::TECHNOLOGY,
                storage: 300,
                daily: 3,
            },
        }
    }
}