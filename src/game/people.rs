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

    pub fn next(&mut self) {
        if self.quantity > 0 && self.quantity < self.max_limit {
            self.process += self.speed;
            if self.process >= 100.0 {
                self.quantity += 1;
                self.process = 0.0;
            }
        }
    }
}
