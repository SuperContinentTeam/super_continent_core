use std::error::Error;
use std::thread;

use chrono::Local;
use tokio;

mod state;

async fn sleep(sec: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
}

fn main() {
    let thread_state = thread::spawn(move || {
        let rt_state = tokio::runtime::Runtime::new().unwrap();

        rt_state.block_on(async {
            println!("启动状态机: {}", Local::now().format("%F %T"));
            let time_flow = tokio::time::Duration::from_secs(1);

            let mut state = state::core::State::new("A".to_string());
            loop {
                println!("State: {}, tick: {}", state.name, state.tick);
                match state.next() {
                    Ok(_) => { tokio::time::sleep(time_flow).await; }
                    Err(_) => { break; }
                }
            }
            println!("结束状态机: {}", Local::now().format("%F %T"));
        });
    });

    thread_state.join().unwrap();
    // let thread_net = thread::spawn(|| {
    //     let rt_net = tokio::runtime::Runtime::new().unwrap();
    // });
    // thread_net.join().unwrap();
}