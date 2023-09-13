use super::resource::StateResource;

pub struct Player {
    pub name: String,
    pub state_resource: StateResource,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state_resource: StateResource::default()
        }
    }

    pub fn dumps(&self) -> String {
        let results = vec![
            self.state_resource.dumps()
        ];
        results.join(";")
    }

    pub fn next(&mut self) {
        self.state_resource.next();
    }
}