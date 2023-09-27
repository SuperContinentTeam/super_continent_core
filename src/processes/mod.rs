pub mod technology;

pub struct Process {
    pub final_point: f64,
    pub schedule: f64,
}

impl Process {
    pub fn new(final_point: f64) -> Self {
        Self {
            final_point,
            schedule: 0.0,
        }
    }

    pub fn finished(&self) -> bool {
        self.schedule >= self.final_point
    }

    pub fn add_point(&mut self, point: f64) {
        self.schedule += point
    }
}
