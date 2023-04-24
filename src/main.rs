use std::net::SocketAddr;

use dotenv::dotenv;
use tokio::net::TcpListener;

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

    connections::initial_peer_map();
    let addr: SocketAddr = "0.0.0.0:55555".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    // WebSocket Server
    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                println!("Peer address: {}", peer);
                tokio::spawn(connections::accept_connection(peer, stream));
            }
            Err(_) => {
                eprintln!("connected streams should have a peer address");
            }
        }
    }
}
