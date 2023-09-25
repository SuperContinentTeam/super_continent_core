use serde_json::{json, Value};

use crate::{
    assets::event::Events,
    cst,
    reference::{random_block_env, random_product, POPULATION_GROWTH},
};

use super::{people::People, units::legion::Legion, Dumps};

pub struct Block {
    pub row: i32,
    pub col: i32,
    pub belong: Option<String>,
    pub environment: i32,
    pub z_width: i32,
    pub zoning_set: Vec<i32>,
    pub people: People,

    // 驻扎军团
    pub legion: Option<Legion>,
    pub product: (f64, f64, f64),
    pub events: Events,
}

impl Block {
    pub fn new(row: i32, col: i32, z_width: i32) -> Self {
        let e = random_block_env();
        Self {
            row,
            col,
            belong: None,
            z_width,
            environment: e,
            zoning_set: Vec::new(),
            people: People::new(z_width),
            legion: None,
            product: random_product(e),
            events: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        self.people.next();
    }

    pub fn initial_people(&mut self) {
        // let m = POPULATION_GROWTH[self.environment as usize + 2];
        self.people.quantity = cst::PLAYER_NEW_BLOCK_PEOPLE;
        // self.people.update(m);
    }

    pub fn can_visit(&self, player: &str) -> bool {
        if let Some(p) = self.belong.as_ref() {
            return p == player;
        }

        false
    }

    // pub fn can_cross(&self, player: &Player) -> bool {
    //     match &self.belong {
    //         Some(p) => &player.name == p,
    //         None => true,
    //     }
    // }

    pub fn set_player(&mut self, player: Option<String>) {
        self.belong = player;
    }
}

impl Dumps for Block {
    fn dumps(&self, player: &str) -> Value {
        json!({
            "row": self.row,
            "col": self.col,
            "env": self.environment,
            "can_visit": if self.can_visit(player) { 1 } else { 0 },
            "can_cross": false,
            "product": self.product,
            "z_width": self.z_width,
            "zoning_set": self.zoning_set,
            "belong": self.belong
        })
    }
}
