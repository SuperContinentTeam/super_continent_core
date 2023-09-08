use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::Local;
use tokio;

mod state;
mod event_bus;


fn main() {
    let name = serde_json::json!({"name": "A"});
    state::manager::add_state(name);

    let name2 = serde_json::json!({"name": "B"});
    state::manager::add_state(name2);

    // let ax_manager = Arc::new(Mutex::new(StateManager::new()));
    // let manager_clone = ax_manager.clone();
    //
    // let thread_state = thread::spawn(move || {
    //     let rt_state = tokio::runtime::Builder::new_current_thread().enable_all().unwrap();
    //
    //     println!("启动状态机: {}", Local::now().format("%F %T"));
    //     rt_state.block_on(async {
    //         let tick = tokio::time::Duration::from_secs(1);
    //
    //         loop {
    //             let mut manager = manager_clone.lock().unwrap();
    //             manager.next().await;
    //             tokio::time::sleep(tick).await;
    //         }
    //         // println!("结束状态机: {}", Local::now().format("%F %T"));
    //     });
    // });
    //
    // thread_state.join().unwrap();
    // let thread_net = thread::spawn(|| {
    //     let rt_net = tokio::runtime::Runtime::new().unwrap();
    // });
    // thread_net.join().unwrap();
}