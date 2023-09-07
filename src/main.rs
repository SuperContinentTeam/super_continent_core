use std::sync::{Arc, Mutex};
use std::thread;

use chrono::Local;
use tokio;

mod state;

use state::manager::StateManager;

fn main() {
    let ax_manager = Arc::new(Mutex::new(StateManager::new()));
    let manager_clone = ax_manager.clone();

    let thread_state = thread::spawn(move || {
        let rt_state = tokio::runtime::Runtime::new().unwrap();

        println!("启动状态机: {}", Local::now().format("%F %T"));
        rt_state.block_on(async {
            let tick = tokio::time::Duration::from_secs(1);

            loop {
                let mut manager = manager_clone.lock().unwrap();
                manager.next().await;
                tokio::time::sleep(tick).await;
            }
            // println!("结束状态机: {}", Local::now().format("%F %T"));
        });
    });

    thread_state.join().unwrap();
    // let thread_net = thread::spawn(|| {
    //     let rt_net = tokio::runtime::Runtime::new().unwrap();
    // });
    // thread_net.join().unwrap();
}