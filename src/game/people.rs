pub struct People {
    pub quantity: i32,
    pub max_limit: i32,
}

impl People {
    pub fn new(quantity: i32, z_width: i32) -> Self {
        Self {
            quantity,
            max_limit: z_width * z_width * 10
        }
    }
}