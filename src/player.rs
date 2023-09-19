use crate::{game::block::Block, reference::AxClient, cst};

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

    pub fn dumps(&self) -> String {
        let results = vec![
            self.state_resource.dumps(),
            self.blocks
                .iter()
                .map(|(r, c)| format!("{},{}", r, c))
                .collect::<Vec<String>>()
                .join(":"),
        ];
        results.join(";")
    }

    pub fn next(&mut self) {
        self.state_resource.next();
    }

    pub fn add_block(&mut self, block: &mut Block) {
        self.blocks.push((block.row, block.col));
        block.belong = Some(self.name.clone());
        self.add_block_product(&block.product);
    }

    pub fn add_block_product(&mut self, product: &(i32, i32, i32)) {
        println!("in add block product");
        let k = cst::BLOCK.to_string();
        let (e, m, f) = product;

        let e_entry = self.state_resource.energy.projects.entry(k.clone()).or_insert(0);
        *e_entry += e;

        let m_entry = self.state_resource.mineral.projects.entry(k.clone()).or_insert(0);
        *m_entry += m;

        let f_entry = self.state_resource.food.projects.entry(k).or_insert(0);
        *f_entry += f;

        self.state_resource.update_daily();
    }

    // pub fn update_player_resource_daily(&self, blocks: Vec<&Block>) {
    //     let mut e = 0;
    //     let mut m = 0;
    //     let mut f = 0;

    //     for block in blocks {
    //         if let Some(belong) = &block.belong {
    //             if belong != &self.name {
    //                 continue;
    //             }

    //             let (ve, vm, vf) = &block.product;
    //             e += ve;
    //             m += vm;
    //             f += vf;
    //         }
    //     }
    // }
}
