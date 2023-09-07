use std::thread;

use tokio;

use chrono::Local;

fn main() {
    let thread_state = thread::spawn(move || {
        let rt_state = tokio::runtime::Runtime::new().unwrap();

        rt_state.block_on(async {
            println!("启动状态机: {}", Local::now().format("%F %T"));
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            println!("结束状态机: {}", Local::now().format("%F %T"));
        });
    });

    thread_state.join().unwrap();
    // let thread_net = thread::spawn(|| {
    //     let rt_net = tokio::runtime::Runtime::new().unwrap();
    // });
    // thread_net.join().unwrap();
}