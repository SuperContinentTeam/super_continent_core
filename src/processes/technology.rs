use crate::{assets::technology::Technology, game::resource::Resource, cst};

use super::Process;

pub struct TechnologyProcess {
    pub process: Process,
    pub name: String,
    pub tech: Technology,
}

impl TechnologyProcess {
    pub fn new(name: String, tech: Technology) -> Self {
        Self {
            name,
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

pub struct TechProcessSolt {
    pub physic: Option<TechnologyProcess>,
    pub civilian: Option<TechnologyProcess>,
    pub extraordinary: Option<TechnologyProcess>,
    // 分配科研重心
    pub tech_points: [f64; 3],
}

impl TechProcessSolt {
    pub fn new() -> Self {
        Self {
            physic: None,
            civilian: None,
            extraordinary: None,
            tech_points: [30.0, 30.0, 40.0],
        }
    }

    pub fn set_technology(&mut self, tech_process: TechnologyProcess ) {
        match tech_process.tech.area.as_str() {
            cst::TECH_AREA_PHYSIC => {
                self.physic = Some(tech_process);
            },
            cst::TECH_AREA_CIVILIAN => {
                self.civilian = Some(tech_process);
            },
            cst::TECH_AREA_EXTRA => {
                self.extraordinary = Some(tech_process);
            }
            _ => {}
        }
    }

    pub fn set_tech_point(&mut self, a: f64, b: f64, c: f64) {
        if ((a + b + c) - 100.0) <= 0.0001 {
            self.tech_points = [a, b, c];
        }
    }

    pub fn next(&mut self, res: &mut Resource) {
        let r: [f64; 3] = [
            self.tech_points[0] / 100.0,
            self.tech_points[1] / 100.0,
            self.tech_points[2] / 100.0,
        ];

        let ps_point = res.storage * r[0];
        let cs_point = res.storage * r[1];
        let es_point = res.storage - ps_point - cs_point;

        let pd_point = res.daily * r[0];
        let cd_point = res.daily * r[1];
        let ed_point = res.daily - pd_point - cd_point;

        let mut o = 0.0;
        if let Some(ph) = self.physic.as_mut() {
            ph.next(ps_point + pd_point);
        } else {
            o += ps_point + pd_point;
        }

        if let Some(ci) = self.civilian.as_mut() {
            ci.next(cs_point + cd_point);
        } else {
            o += cs_point + cd_point;
        }

        if let Some(ex) = self.extraordinary.as_mut() {
            ex.next(es_point + ed_point);
        } else {
            o += es_point + ed_point;
        }

        res.storage = o;
    }

    pub fn check_finish(&mut self) -> Option<Vec<String>> {
        let mut result = Vec::new();
        if let Some(i) = &self.physic {
            if i.process.finished() {
                result.push(i.name.clone());
                self.physic = None;
            }
        }

        if let Some(i) = &self.civilian {
            if i.process.finished() {
                result.push(i.name.clone());
                self.civilian = None;
            }
        }

        if let Some(i) = &self.extraordinary {
            if i.process.finished() {
                result.push(i.name.clone());
                self.extraordinary = None;
            }
        }
        
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}
