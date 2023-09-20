pub struct Zoning {
    pub row: i32,
    pub col: i32,
    pub block_pos: (i32, i32),
}


impl Zoning {
    pub fn new(row: i32, col: i32, block_pos: (i32, i32)) -> Self {
        Self {
            row,
            col,
            block_pos,
        }
    }
}