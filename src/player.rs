use serde_json::json;

use crate::{
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
}

impl Player {
    pub fn new(client: AxClient, name: String) -> Self {
        Self {
            client,
            name,
            ready: 0,
            state_resource: StateResource::new(),
            blocks: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        self.state_resource.next();
    }

    pub fn add_block(&mut self, block: &mut Block) {
        self.blocks.push((block.row, block.col));
        block.belong = Some(self.name.clone());
        self.add_block_product(&block.product);
    }

    // pub fn remove_block(&mut self, block: &mut Block) {
    //     self.blocks.retain(|x| x != &(block.row, block.col));
    //     block.belong = None;
    //     let (e, m, f) = block.product;
    //     self.add_block_product(&(-e, -m, -f));
    // }

    pub fn add_block_product(&mut self, product: &(i32, i32, i32)) {
        let (e, m, f) = product;
        self.state_resource.add_block_product(cst::ENERGY, *e);
        self.state_resource.add_block_product(cst::MINERAL, *m);
        self.state_resource.add_block_product(cst::FOOD, *f);
    }
}

impl Dumps for Player {
    fn dumps(&self, player: &str) -> serde_json::Value {
        json!({
            cst::DUMP_PLAYER_RESOURCE: self.state_resource.dumps(player),
            cst::DUMP_BLOCK: self.blocks
        })
    }
}
