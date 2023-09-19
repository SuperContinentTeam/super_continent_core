use crate::reference::AxClient;

use super::game::resource::StateResource;

pub struct Player{
    pub client: AxClient,
    pub ready: i32, // 0: not ready, 1: ready, 2: observe
    pub name: String,
    pub state_resource: StateResource,
    pub blocks: Vec<(i32, i32)>,
}

impl Player {
    pub fn new(client: AxClient, name: String) -> Self {
        Self {
            client,
            name,
            ready: 0,
            state_resource: StateResource::new(),
            blocks: Vec::new()
        }
    }


    pub fn dumps(&self) -> String {
        let results = vec![
            self.state_resource.dumps(),
            self.blocks.iter().map(|(r,c)|{format!("{},{}",r,c)}).collect::<Vec<String>>().join(":"),
        ];
        results.join(";")
    }

    pub fn next(&mut self) {
        self.state_resource.next();
    }
}