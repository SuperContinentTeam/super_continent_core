use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::thread;
use tokio::{
    net::{TcpListener, TcpStream},
    runtime::Runtime,
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{protocol::Message, Error, Result},
};
use dotenv::dotenv;
use crate::state::state::get_game_state;

mod game;
mod redis_client;

mod state {
    pub mod state;
    pub mod tick;
}

mod room {
    pub mod room;
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr: SocketAddr = "0.0.0.0:55555".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    // Game Main Loop
    let _ = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(game::game_loop(get_game_state()));
    });

    // WebSocket Server
    let websocket_server = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(connect_from_client(listener));
    });

    websocket_server.join().unwrap();
}

async fn connect_from_client(listener: TcpListener) {
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, stream));
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Get Message: {}", text);

                let message = game::command_executor(get_game_state(), &text).await;
                println!("Result message: {}", message);

                ws_stream
                    .send(Message::Text(format!("{}", message)))
                    .await?
            }
            Ok(Message::Close(_)) => {
                println!("Client disconnect");
                break;
            }
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
