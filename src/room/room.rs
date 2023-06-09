// use std::collections::HashMap;
// use serde::{Serialize, Deserialize};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Player {
//     pub name: String,
//     pub entity: String,
//     pub room_entity: Option<String>,
// }

// impl Player {
//     pub fn new(name: String, entity: String, room_entity: Option<String>) -> Self {
//         Self {
//             name,
//             entity,
//             room_entity,
//         }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Room {
//     pub name: String,
//     pub entity: String,
//     pub users: HashMap<String, Player>,
// }

// impl Room {
//     pub fn new(name: String, entity: String) -> Self {
//         Self {
//             name,
//             entity,
//             users: HashMap::new(),
//         }
//     }

//     pub fn join(&mut self, mut player: Player) {
//         if !self.users.contains_key(player.entity.as_str()) {
//             let entity = player.entity.clone();
//             self.users.insert(entity, player.clone());
//             player.room_entity = Some(self.entity.clone());
//         }
//     }

//     pub fn leave(&mut self, mut player: Player) {
//         let key = player.entity.as_str();
//         if self.users.contains_key(key) {
//             self.users.remove(key);
//             player.room_entity = None;
//         }
//     }

// }
