use crate::meta::parse_toml_config;
use game::world::World;
use meta::Configure;
use reference::AXState;
use state::{run_state, State};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

mod commander;
mod game;
mod meta;
mod player;
mod reference;
mod state;
mod ws;
mod cst;

fn main() {
    let conf = parse_toml_config();
    let ws_addr = conf.ws_server.clone();

    let ax_state = new_state_from_meta(&conf);

    let s1 = ax_state.clone();
    let t1 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_websocket_server(ws_addr, s1));
    });

    let s2 = ax_state.clone();
    let t2 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_state(s2));
    });

    let _ = t1.join();
    let _ = t2.join();
}

async fn start_websocket_server(addr: String, s: AXState) {
    println!("WebSocket Server Listening on: {}", addr);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(ws::handle_connection(stream, addr, s.clone()));
    }
}

fn new_state_from_meta(conf: &Configure) -> AXState {
    let s = State {
        tick: 0,
        players: HashMap::new(),
        admin: String::new(),
        max_player: conf.max_player,
        status: 0,
        world: World::new(conf.world_size),
    };

    Arc::new(Mutex::new(s))
}
