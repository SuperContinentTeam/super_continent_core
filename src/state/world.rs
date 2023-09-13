use rand::Rng;
use std::collections::HashMap;

use crate::reference::random_block_env;

// use super::player::Player;

pub struct Block {
    pub row: i32,
    pub col: i32,
    pub belong: Option<String>,
    pub environment: i32,
}

impl Block {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            belong: None,
            environment: random_block_env()
        }
    }

    // pub fn can_cross(&self, player: &Player) -> bool {
    //     match &self.belong {
    //         Some(p) => &player.name == p,
    //         None => true,
    //     }
    // }
}

pub struct World {
    pub width: i32,
    pub blocks: HashMap<(i32, i32), Block>,
}

impl World {
    pub fn new(width: i32) -> Self {
        let mut blocks: HashMap<(i32, i32), Block> = HashMap::new();
        for row in 0..width {
            for col in 0..width {
                blocks.insert((row, col), Block::new(row, col));
            }
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
                if let Some(b) = self.blocks.get(&(r, c)) {
                    if b.belong.is_some() {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn rand_block(&self) -> (i32, i32) {
        let mut r_row = rand::thread_rng().gen_range(0..self.width);
        let mut r_col = rand::thread_rng().gen_range(0..self.width);
        let mut count = 100;
        while count > 0 {
            if self.no_neighbor(r_row, r_col) {
                break;
            }
            r_row = rand::thread_rng().gen_range(0..self.width);
            r_col = rand::thread_rng().gen_range(0..self.width);
            count -= 1;
        }

        (r_row, r_col)
    }
}
