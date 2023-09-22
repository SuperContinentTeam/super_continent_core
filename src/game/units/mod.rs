pub mod engineer;
pub mod explorer;
pub mod legion;
pub mod soldier;
pub mod unit_cst;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::game::units::soldier::Soldier;

use self::unit_cst::{UNIT_DODGE, UNIT_HEALTH, UNIT_HEALTH_RECOVERY, UNIT_MOVE_SPEED};

lazy_static! {
    // 士兵表 {Player: {code: Solider}}
    pub static ref SOLDIERS: HashMap<String, HashMap<i32, Soldier>> = HashMap::new();
}

pub struct UnitMeta {
    // 编号
    pub code: u64,
    // 阵营
    pub camp: String,
    // 驻地
    pub station: (i32, i32),
    // 生命
    pub health: f64,
    // 生命恢复速度(/tick)
    pub health_recovery: f64,
    // 闪避 dodge=10 -> 10%
    pub dodge: f64,
    // 移速 (tick/block)
    pub move_speed: f64,
    // 等级
    // pub level: u8,
    // 当前经验值
    // pub exp: i64,
    // 下一级所需经验值
    // pub next_need: i64,
}

impl UnitMeta {
    pub fn new(code: u64, player: &str, station: (i32, i32)) -> Self {
        Self {
            code: code as u64,
            camp: player.to_string(),
            station,
            health: UNIT_HEALTH,
            health_recovery: UNIT_HEALTH_RECOVERY,
            dodge: UNIT_DODGE,
            move_speed: UNIT_MOVE_SPEED,
        }
    }
}
