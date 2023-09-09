mod state;
mod event_bus;


fn main() {
    let name1 = serde_json::json!({"name": "A"});
    let name2 = serde_json::json!({"name": "B"});

    event_bus::register("AddState", |value| {
        state::manager::add_state(value);
    });

    event_bus::emit("AddState", &name1);
    event_bus::emit("AddState", &name2);

    std::thread::sleep(std::time::Duration::from_secs(3600));

    // let thread_net = thread::spawn(|| {
    //     let rt_net = tokio::runtime::Runtime::new().unwrap();
    // });
    // thread_net.join().unwrap();
}