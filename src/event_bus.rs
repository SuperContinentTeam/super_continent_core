use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

type Listener = fn(serde_json::Value);
type Listeners = Vec<Listener>;
type EventMap = Arc<Mutex<HashMap<String, Listeners>>>;

lazy_static! {
    static ref EVENT_MAP: Arc<Mutex<HashMap<String, Listeners>>> = EventMap::default();
}


pub fn register(name: &str, listener: Listener) {
    let event_clone = EVENT_MAP.clone();
    let mut event_map = event_clone.lock().unwrap();

    match event_map.get_mut(name) {
        None => {
            let listeners = vec![listener];
            event_map.insert(name.to_string(), listeners);
        }
        Some(listeners) => {
            listeners.push(listener);
        }
    }
}

pub fn emit(name: &str, value: &serde_json::Value) {
    let event_clone = EVENT_MAP.clone();
    let event_map = event_clone.lock().unwrap();

    if let Some(listeners) = event_map.get(name) {
        for listener in listeners {
            listener(value.clone());
        }
    }
}
