mod commander;
mod state;
mod ws;
mod db;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(ws::start_server());
}

