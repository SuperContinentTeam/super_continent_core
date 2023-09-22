use rand::Rng;

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

    pub fn battle(legion1: &mut Self, legion2: &mut Self) {
        let mut rng = rand::thread_rng();

        let legion1_len = legion1.soliders.len();
        let legion2_len = legion2.soliders.len();

        for friend in &legion1.soliders {
            let enemy = legion2
                .soliders
                .get_mut(rng.gen_range(0..legion2_len + 1))
                .unwrap();
            friend.attack_to(enemy);
        }

        for enemy in &legion2.soliders {
            let friend = legion1
                .soliders
                .get_mut(rng.gen_range(0..legion1_len + 1))
                .unwrap();
            enemy.attack_to(friend);
        }
    }

    pub fn is_survive(&self) -> bool {
        for soldier in &self.soliders {
            if soldier.is_survive() {
                return true;
            }
        }

        false
    }

    pub fn get_power(&self) -> f64 {
        self.soliders.iter().map(|x| x.get_power()).sum()
    }
}
