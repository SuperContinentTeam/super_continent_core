use serde_json::json;

use crate::{
    assets::{event::Events, EVENTS},
    cst,
    game::{block::Block, Dumps},
    reference::AxClient,
};

use super::game::resource::StateResource;

pub struct Player {
    pub client: AxClient,
    pub ready: i32, // 0: not ready, 1: ready, 2: observe
    pub name: String,
    pub state_resource: StateResource,
    pub blocks: Vec<(i32, i32)>,

    pub events: Events,
}

impl Player {
    pub fn new(client: AxClient, name: String) -> Self {
        Self {
            client,
            name,
            ready: 0,
            state_resource: StateResource::new(),
            blocks: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        self.state_resource.next();
    }

    pub fn add_block(&mut self, block: &mut Block) {
        self.blocks.push((block.row, block.col));
        block.belong = Some(self.name.clone());
        self.update_daily_with_block(block, true);
    }

    pub fn update_daily_with_block(&mut self, block: &Block, is_add: bool) {
        let neg = if is_add { 1.0 } else { -1.0 };
        let v = (
            self.get_value_with_modifier(cst::BLOCK, cst::ENERGY, block.product.0 * neg),
            self.get_value_with_modifier(cst::BLOCK, cst::MINERAL, block.product.1 * neg),
            self.get_value_with_modifier(cst::BLOCK, cst::FOOD, block.product.2 * neg),
        );

        self.state_resource.update_daily_with_block(v);
    }

    pub fn get_value_with_modifier(&self, code: &str, entity: &str, value: f64) -> f64 {
        let mut a1 = value;
        let mut a2 = 1.0;

        for event in &self.events {
            for modifier in &EVENTS[event] {
                if modifier.code == code && modifier.entity == entity {
                    if modifier.method == cst::MODIFIER_METHOD_ADD {
                        a1 += modifier.value;
                    }
                    if modifier.method == cst::MODIFIER_METHOD_MUL {
                        a2 *= 1.0 + modifier.value;
                    }
                }
            }
        }

        a1 * a2
    }

    // pub fn remove_block(&mut self, block: &mut Block) {
    //     self.blocks.retain(|x| x != &(block.row, block.col));
    //     block.belong = None;
    //     let (e, m, f) = block.product;
    //     self.add_block_product(&(-e, -m, -f));
    // }
}

impl Dumps for Player {
    fn dumps(&self, player: &str) -> serde_json::Value {
        json!({
            cst::DUMP_PLAYER_RESOURCE: self.state_resource.dumps(player),
            cst::DUMP_BLOCK: self.blocks
        })
    }
}
