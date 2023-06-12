use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

type ArcMutex<T> = Arc<Mutex<T>>;
pub struct State {
    pub name: String,
    pub tick: u8,
    pub pause: bool,
}

pub struct StateManager {
    pub state_map: ArcMutex<HashMap<String, ArcMutex<State>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_state(&self, name: String) {
        self.state_map
            .clone()
            .lock()
            .unwrap()
            .insert(name.clone(), Arc::new(Mutex::new(State::new(name.clone()))));
    }

    pub fn start(&self, name: String) {
        if let Some(state) = self.state_map.clone().lock().unwrap().get(&name) {
            let state_clone = state.clone();
            thread::spawn(move || {
                println!("启动State: {}", name);
                run(state_clone);
            });
        }
    }
}

impl State {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tick: 0,
            pause: true,
        }
    }
}

fn sleep(duration: u64) {
    thread::sleep(Duration::from_secs(duration));
}

fn run(ax_state: Arc<Mutex<State>>) {
    let mux_state = ax_state.clone();
    loop {
        sleep(1);
        let mut state = mux_state.lock().unwrap();
        // 是否暂停
        if state.pause {
            continue;
        }
        println!("Tick is: {}", state.tick);
        state.tick += 1;
    }
}
