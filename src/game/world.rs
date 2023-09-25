use std::collections::HashMap;

use serde_json::{json, Value};

use crate::cst;
use crate::game::block::Block;
use crate::game::zoning::Zoning;
use crate::reference::random_between;

use super::Dumps;
pub type BlockMap = HashMap<(i32, i32), Block>;
pub type ZoningMap = HashMap<(i32, i32, i32), Zoning>;

pub struct World {
    pub width: i32,
    pub blocks: BlockMap,
    pub zoning_map: ZoningMap,
}

fn initial_world(width: i32) -> (BlockMap, ZoningMap) {
    let mut blocks: BlockMap = BlockMap::default();
    let mut zoning_map: ZoningMap = ZoningMap::default();

    for row in 0..width {
        for col in 0..width {
            blocks.insert((row, col), Block::new(row, col, random_between(3, 6)));
        }
    }

    let mut z_index = 0;
    for (b_pos, block) in blocks.iter_mut() {
        for zr in 0..block.z_width {
            for zc in 0..block.z_width {
                zoning_map.insert((z_index, zr, zc), Zoning::new(zc, zr, *b_pos));
                block.zoning_set.push(z_index);

                z_index += 1;
            }
        }
    }

    (blocks, zoning_map)
}

impl World {
    pub fn new(width: i32) -> Self {
        let (blocks, zoning_map) = initial_world(width);
        Self {
            width,
            blocks,
            zoning_map,
        }
    }

    pub fn next(&mut self) {
        for (_, block) in &mut self.blocks {
            block.next();
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

        for pos in &neighbors {
            if 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.width {
                let b = &self.blocks[pos];
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

    pub fn query_blocks(&self, name: Option<&str>) -> Vec<&Block> {
        let mut v = Vec::new();
        for b in self.blocks.values() {
            v.push(b);
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
