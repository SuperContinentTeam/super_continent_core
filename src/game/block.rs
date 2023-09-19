use crate::reference::random_block_env;

pub struct Block {
    pub row: i32,
    pub col: i32,
    pub belong: Option<String>,
    pub environment: i32,
    pub z_width: i32,
    pub zoning_set: Vec<i32>,
}

impl Block {
    pub fn new(row: i32, col: i32, z_width: i32) -> Self {
        Self {
            row,
            col,
            belong: None,
            z_width,
            environment: random_block_env(),
            zoning_set: Vec::new(),
        }
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
