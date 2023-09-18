use std::{net::SocketAddr, str::FromStr};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

mod commander;
mod db;
mod state;
mod ws;
mod reference;
mod http_server;
mod meta;

fn main() {
    let t1 = std::thread::spawn(|| {
        let rt =  tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_websocket_server());
    });

    let t2 = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_http_server());
    });

    let _ = t1.join();
    let _ = t2.join();
}

async fn start_websocket_server() {
    let addr = "0.0.0.0:10000".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    println!("WebSocket Server Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(ws::handle_connection(stream, addr));
    }
}

async fn start_http_server() {
    let addr = "0.0.0.0:10001";
    let sock = SocketAddr::from_str(addr).expect("Failed to bind");

    println!("Http Server Listening on: {}", addr);

    let app = http_server::build_router()
        .layer(CorsLayer::new());
    let _ = axum::Server::bind(&sock).serve(app.into_make_service()).await;
}