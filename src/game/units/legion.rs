use super::{soldier::Soldier, unit_cst::LEGION_CAPACITY};

pub struct Legion {
    pub name: String,
    pub capacity: u64,
    pub soliders: Vec<Soldier>,
}

impl Legion {
    pub fn new(name: &str, solider: Soldier) -> Self {
        Self {
            name: name.to_string(),
            capacity: LEGION_CAPACITY,
            soliders: vec![solider],
        }
    }

    pub fn merge(&mut self, mut legion: Self) {
        let mut code: u64 = 1;
        let mut temp_solidiers = std::mem::take(&mut legion.soliders);
        self.soliders.extend(temp_solidiers);

        for v in &mut self.soliders {
            v.meta.code = code;
            code += 1;
        }
    }
}
