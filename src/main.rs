use tokio::net::TcpListener;

mod commander;
mod db;
mod state;
mod ws;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start_server());
}

async fn start_server() {
    let addr = "0.0.0.0:7000".to_string();

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(ws::handle_connection(stream, addr));
    }
}
