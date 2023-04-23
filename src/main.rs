use std::net::SocketAddr;
use std::thread;
use tokio::{
    net::TcpListener,
    runtime::Runtime,
};
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr: SocketAddr = "0.0.0.0:55555".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    // let console = thread::spawn(move || {
    //     let rt = Runtime::new().unwrap();
    //     rt.block_on(move || async {
    //         let stdin = io::stdin();
    //         loop {
    //             let mut line = String::new();
    //             stdin.lock().read_line(&mut line).unwrap();
    //             println!("You Enter: {}", line);
    //         }
    //     })
    // });

    // Game Main Loop
    // let _ = thread::spawn(move || {
    //     let rt = Runtime::new().unwrap();
    //     rt.block_on(game::game_loop(get_game_state()));
    // });

    // WebSocket Server
    connections::initial_peer_map();
    let websocket_server = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(connections::connect_from_client(listener));
    });

    websocket_server.join().unwrap();
}



