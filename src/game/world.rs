use crate::reference::random_between;

use super::block::Block;

// use super::player::Player;

pub struct World {
    pub width: i32,
    pub blocks: Vec<Vec<Block>>,
}

impl World {
    pub fn new(width: i32) -> Self {
        let mut blocks = Vec::new();
        for row in 0..width {
            let mut v = Vec::new();
            for col in 0..width {
                v.push(Block::new(row, col));
            }
            blocks.push(v);
        }

        Self { width, blocks }
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
}
