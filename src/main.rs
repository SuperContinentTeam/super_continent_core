mod commander;
mod state;
mod ws;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(ws::start_server());
}

