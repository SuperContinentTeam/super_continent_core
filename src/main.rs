mod event_bus;
mod state;

fn main() {
    event_bus::register("AddState", |value| {
        state::manager::add_state(value);
    });

    for i in 1..3 {
        let name = serde_json::json!({"name": i});
        event_bus::emit("AddState", &name);
    }

    // 网络通讯运行时
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let mut time_counter = 1;
        loop {
            println!("等待连接: {}", time_counter);
            time_counter += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
}
