use std::{net::SocketAddr, thread};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use dotenv::dotenv;
// use crate::state::state::get_game_state;

mod game;
mod redis_client;

mod state {
    pub mod state;
    pub mod tick;
}

mod room {
    pub mod room;
}

mod connections;
mod queue;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let running = Arc::new(Mutex::new(true));

    // Game Main Loop
    // let _ = thread::spawn(move || {
    //     let rt = Runtime::new().unwrap();
    //     rt.block_on(game::game_loop(get_game_state()));
    // });

    // WebSocket Server
    let running_websocket_server = Arc::clone(&running);
    connections::initial_peer_map();
    let websocket_server = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(connections::connect_from_client(running_websocket_server));
    });

    websocket_server.join().unwrap();
}



