pub const CHANGE_TECH_RATE: &str = "change_tech_rate";

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub create_tick: u64,
    pub lifetime: Option<u64>,
    pub tick: u64,
}

impl Tag {
    pub fn new(name: String, create_tick: u64, lifetime: Option<u64>) -> Self {
        Self {
            name,
            create_tick,
            lifetime,
            tick: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(lifetime) = self.lifetime {
            self.tick >= lifetime
        } else {
            false
        }
    }

    pub fn next(&mut self) {
        self.tick += 1;
    }
}
