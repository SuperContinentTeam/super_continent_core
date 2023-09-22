use std::result;

use rand::Rng;

use super::{
    unit_cst::{SOLDIER_ATTACK_SPEED, SOLDIER_KINETIC_ATTACK, SOLDIER_MAX_WEIGHT},
    UnitMeta, SOLDIERS,
};
// 攻击
pub struct Attack {
    pub energy: f64,
    pub kinetic: f64,
    pub speed: f64,
}

// 防御
pub struct Defense {
    pub energy: f64,
    pub kinetic: f64,
}

impl Defense {
    pub fn new() -> Self {
        Self {
            energy: 0.0,
            kinetic: 0.0,
        }
    }

    pub fn get_exemption(&self, c: f64) -> (f64, f64) {
        let k  = self.kinetic / (c + self.kinetic);
        let e = self.energy / (c + self.energy);
        (k, e)
    }
}

// 士兵
pub struct Soldier {
    pub meta: UnitMeta,
    pub legion: String,
    // 负重 (kg)
    pub weight: f64,
    // 负重上限
    pub max_weight: f64,

    pub attack: Attack,
    pub defense: Defense,
}

impl Soldier {
    pub fn new(code: u64, player: &str, legion: &str, station: (i32, i32)) -> Self {
        Self {
            meta: UnitMeta::new(code, player, station),
            legion: legion.to_string(),
            weight: 0.0,
            max_weight: SOLDIER_MAX_WEIGHT,
            attack: Attack {
                energy: SOLDIER_KINETIC_ATTACK,
                kinetic: 0.0,
                speed: SOLDIER_ATTACK_SPEED,
            },
            defense: Defense::new(),
        }
    }

    pub fn get_exemption(&self) -> (f64, f64) {
        let c = self.meta.health * 0.5;
        self.defense.get_exemption(c)
    }

    pub fn attack_to(&self, other: &mut Self) {
        // 闪避判定 b * ( 1 + (b-a)/b )
        let using_dodge = 2.0 * other.meta.dodge - self.meta.dodge;
        let mut rng = rand::thread_rng();

        // 闪避成功
        if rng.gen_range(0.0..1.0) < (using_dodge / 100.0) {
            return;
        }

        // 计算伤害减免
        let (k_exemption, e_exemption) = other.get_exemption();
        // 实际伤害
        let dam_kinetic = self.attack.kinetic * (1.0 - k_exemption);
        let dam_energy = self.attack.energy * (1.0 - e_exemption);

        other.meta.health -= dam_energy + dam_kinetic;
    }

    pub fn is_survive(&self) -> bool {
        self.meta.health > 0.0
    }

    pub fn get_power(&self) -> f64 {
        // P = HP * (1 - E(kinetic)) * (1 - E(energy)) * (Atk(kinetic) + Atk(energy))
        let quantify_damage_power = self.attack.energy + self.attack.kinetic;
        let quantity_defense = self.get_exemption();

        self.meta.health * (1.0 - quantity_defense.0) * (1.0 - quantity_defense.1) * quantify_damage_power
    }
}
