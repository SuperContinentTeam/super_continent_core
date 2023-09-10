mod event_bus;
mod state;
mod ws;

fn main() {
    event_bus::register("AddState", |value| {
        state::manager::add_state(value);
    });

    // 网络通讯运行时
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(ws::start_server());
}