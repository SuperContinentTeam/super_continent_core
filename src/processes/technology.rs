use crate::assets::technology::Technology;

use super::Process;

pub struct TechnologyProcess {
    pub process: Process,
    pub tech: Technology,
}

impl TechnologyProcess {
    pub fn new(tech: Technology) -> Self {
        Self {
            process: Process::new(tech.cost as f64),
            tech: tech.clone(),
        }
    }

    pub fn next(&mut self, point: f64) {
        if !self.process.finished() {
            self.process.add_point(point);
        }
    }
}
