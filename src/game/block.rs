use crate::reference::random_block_env;
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
            environment: random_block_env(),
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
