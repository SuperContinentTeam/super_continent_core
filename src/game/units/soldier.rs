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
}
