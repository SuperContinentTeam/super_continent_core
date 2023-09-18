use std::net::SocketAddr;

use crate::reference::AxClient;

use super::game::resource::StateResource;

pub struct Player{
    pub client: AxClient,
    pub ready: bool,
    pub name: String,
    pub state_resource: StateResource,
    pub blocks: Vec<(i32, i32)>,
}

impl Player {
    pub fn new(client: AxClient, name: String) -> Self {
        Self {
            client,
            name,
            ready: false,
            state_resource: StateResource::default(),
            blocks: Vec::new()
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