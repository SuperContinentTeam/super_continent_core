use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{
    assets::{event::Events, EVENTS, tags::Tag},
    cst,
    game::{
        block::Block,
        units::{legion::Legion, soldier::Soldier},
        Dumps,
    },
    reference::AxClient, processes::technology::TechProcessSolt, ws::send_message,
};

use super::game::resource::StateResource;

pub struct Player {
    pub client: AxClient,
    pub ready: i32, // 0: not ready, 1: ready, 2: observe
    pub name: String,
    pub state_resource: StateResource,
    pub blocks: Vec<(i32, i32)>,
    pub tech_process_sot: TechProcessSolt,
    // 全局修正buff
    pub events: Events,
    // 全局标签
    pub tags: Vec<Tag>,
    // 已完成科技
    pub finished_techs: Vec<String>,
    // 军团
    pub legions: HashMap<String, Legion>,

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
            legions: HashMap::new(),
            finished_techs: Vec::new(),
            tags: Vec::new(),
            tech_process_sot: TechProcessSolt::new(),
        }
    }

    pub fn next(&mut self) {
        self.state_resource.next();
        self.tech_process_sot.next(self.state_resource.resource_map.get_mut(cst::TECHNOLOGY).unwrap());

        for tag in &mut self.tags {
            tag.next();
        }
        self.tags.retain(|x| !x.is_expired());
    }

    pub fn build_solider(&mut self, block: &mut Block) {
        // 当前地块驻扎有军团 新建士兵加入到该军团
        if let Some(legion) = block.legion.as_mut() {
            let code = legion.soliders.len() + 1;
            let soldier = Soldier::default(
                code as u64,
                &self.name,
                &legion.name,
                (block.row, block.col),
            );
            legion.soliders.push(soldier);
        } else {
            // 当前地块未驻扎军团 新建士兵后新建军团
            let name = format!("第{}军团", self.legions.len() + 1);
            let solider = Soldier::default(0, &self.name, &name, (block.row, block.col));
            let leigon = Legion::new(&name, solider);
            block.legion = Some(leigon);
        }
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

    pub fn has_tag(&self, name: &str) -> bool {
        for tag in &self.tags {
            if tag.name == name {
                return true;
            }
        }

        false
    }

    pub async fn after_next(&mut self) {
        let mut message: HashMap<String, Value> = HashMap::new();
        if let Some(techs) = self.tech_process_sot.check_finish(){
            message.insert("finished_tech".to_string(), json!(techs));
        }

        send_message(json!(message).to_string(), self.client.clone()).await;
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
