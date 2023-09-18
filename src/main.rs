use std::{net::SocketAddr, str::FromStr};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use crate::meta::parse_toml_config;
// use crate::meta::{get_http_addr, get_ws_addr};

mod commander;
mod db;
mod game;
mod ws;
mod reference;
mod http_server;
mod state;
mod player;
mod meta;

fn main() {
    let conf = parse_toml_config();
    let http_addr = conf.http_server.clone();
    let ws_addr = conf.ws_server.clone();

    let t1 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_websocket_server(ws_addr));
    });

    let t2 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_http_server(http_addr));
    });

    let _ = t1.join();
    let _ = t2.join();
}

async fn start_websocket_server(addr: String) {
    println!("WebSocket Server Listening on: {}", addr);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");


    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(ws::handle_connection(stream, addr));
    }
}

async fn start_http_server(addr: String) {
    println!("Http Server Listening on: {}", addr);
    let sock = SocketAddr::from_str(&addr).expect("Failed to bind");
    let app = http_server::build_router()
        .layer(CorsLayer::new());
    let _ = axum::Server::bind(&sock).serve(app.into_make_service()).await;
}
