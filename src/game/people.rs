use crate::cst;

pub struct People {
    pub quantity: i32,
    pub max_limit: i32,
    pub idle: i32,
    pub speed: f64,
    pub process: f64,
}

impl People {
    pub fn new(z_width: i32) -> Self {
        Self {
            quantity: 0,
            max_limit: z_width * z_width * 10,
            idle: 0,
            speed: cst::DEFAULT_POP_SPEED,
            process: 0.0,
        }
    }

    // pub fn update(&mut self, modifier: f64) {
    //     // self.speed = pop_growth(self.quantity as f64, self.max_limit as f64, modifier);
    //     self.process = 0.0;
    // }
}
