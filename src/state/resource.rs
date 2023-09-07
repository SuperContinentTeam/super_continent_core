pub enum ResourceType {
    TICK,
    ENERGY,
    MINERAL,
    FOOD,
    CUSTOMER,
    ALLOY,
    PHYSICS,
    ENGINEER,
    BEYOND,
}

pub struct Resource {
    pub typ: ResourceType,
    pub material: Option<Vec<Self>>,
}

pub struct StateResource {
    pub resource: Resource,
    pub storage: f64,
    pub daily: f64,
}