use std::collections::HashMap;

use serde_json::{json, Value};

use crate::cst;
use crate::game::block::Block;
use crate::game::zoning::Zoning;
use crate::reference::random_between;

use super::Dumps;

pub type BlockSet = Vec<Vec<Block>>;

pub struct World {
    pub width: i32,
    pub blocks: BlockSet,
    pub zoning_set: Vec<Zoning>,
}

fn initial_world(width: i32) -> (BlockSet, Vec<Zoning>) {
    let mut blocks: BlockSet = BlockSet::default();
    let mut zoning_set: Vec<Zoning> = Vec::new();

    for row in 0..width {
        let mut v = Vec::new();
        for col in 0..width {
            let mut b = Block::new(row, col, random_between(3, 6));

            let mut z_index = 0;
            for zr in 0..b.z_width {
                for zc in 0..b.z_width {
                    zoning_set.push(Zoning::new(zr, zc, &b));
                    b.zoning_set.push(z_index);

                    z_index += 1;
                }
            }

            v.push(b);
        }
        blocks.push(v);
    }

    (blocks, zoning_set)
}

impl World {
    pub fn new(width: i32) -> Self {
        let (blocks, zoning_set) = initial_world(width);
        Self {
            width,
            blocks,
            zoning_set,
        }
    }

    fn no_neighbor(&self, r: i32, c: i32) -> bool {
        let neighbors = vec![
            (r - 1, c - 1),
            (r, c - 1),
            (r + 1, c - 1),
            (r - 1, c),
            (r + 1, c),
            (r - 1, c + 1),
            (r, c + 1),
            (r + 1, c + 1),
        ];

        for (r, c) in neighbors {
            if 0 <= r && r < self.width && 0 <= c && c < self.width {
                let b = self.rc_block(r, c);
                if b.belong.is_some() {
                    return false;
                }
            }
        }

        true
    }

    pub fn rand_block(&self) -> (i32, i32) {
        let mut r_row = random_between(0, self.width);
        let mut r_col = random_between(0, self.width);

        let mut count = self.width;
        while count > 0 {
            if self.no_neighbor(r_row, r_col) {
                break;
            }
            r_row = random_between(0, self.width);
            r_col = random_between(0, self.width);
            count -= 1;
        }

        (r_row, r_col)
    }

    pub fn rc_block(&self, r: i32, c: i32) -> &Block {
        &self.blocks[r as usize][c as usize]
    }

    pub fn rc_block_mut(&mut self, r: i32, c: i32) -> &mut Block {
        let vr = self.blocks.get_mut(r as usize).unwrap();
        vr.get_mut(c as usize).unwrap()
    }

    pub fn query_blocks(&self, name: Option<&str>) -> Vec<&Block> {
        let mut v = Vec::new();
        for vr in &self.blocks {
            for block in vr {
                v.push(block);
            }
        }

        if let Some(player) = name {
            v.retain(|x| {
                if let Some(belong) = &x.belong {
                    belong == player
                } else {
                    false
                }
            })
        }

        v
    }

    pub fn dumps_for_blocks(&self, player: &str) -> Value {
        let v = self
            .query_blocks(None)
            .iter()
            .map(|b| (format!("{},{}", b.row, b.col), b.dumps(player)))
            .collect::<HashMap<String, Value>>();

        json!(v)
    }
}

impl Dumps for World {
    fn dumps(&self, player: &str) -> Value {
        json!({
            cst::DUMP_BLOCK: self.dumps_for_blocks(player)
        })
    }
}
