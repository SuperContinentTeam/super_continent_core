pub mod events;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

type Listener = fn(serde_json::Value);
type Listeners = Vec<Listener>;

lazy_static! {
    static ref EVENT_MAP: Arc<Mutex<HashMap<String, Vec<Listener>>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn register(name: String, listener: Listener) {
    let event_clone = EVENT_MAP.clone();
    let mut event_map = event_clone.lock().unwrap();
    match event_map.get_mut(&name) {
        None => {}
        Some(_) => {}
    }
}

// pub fn register<T>(&mut self, name: String, listener: Listener)
// {
//     match self.collections.get_mut(&name) {
//         Some(listeners) => {
//             listeners.push(listener);
//         }
//         None => {
//             let listeners = vec![listener];
//             self.collections.insert(name, listeners);
//         }
//     }
// }
//
// pub fn emit(&self, name: String, value: serde_json::Value) {
//     if let Some(listeners) = self.collections.get(&name) {
//         for listener in listeners {
//             listener(value.clone());
//         }
//     }
// }
// }