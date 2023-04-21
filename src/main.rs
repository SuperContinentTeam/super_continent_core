use futures_util::{SinkExt, StreamExt};
use once_cell::sync::OnceCell;
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

mod game;
mod state {
    pub mod state;
    pub mod tick;
}

use state::state::GameState;

static GAME_STATE: OnceCell<GameState> = OnceCell::new();

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:55555".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    // Global Game State
    let game_state = GameState::new();
    GAME_STATE.set(game_state).unwrap();

    // Game Main Loop
    let _ = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(game::game_loop(GAME_STATE.get().unwrap()));
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

    let game_state = GAME_STATE.get().unwrap();

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Get Message: {}", text);
                let message = {
                    if text == "read_time" {
                        let tick = game_state.tick.read().unwrap();
                        let (year, month, day) = tick.datetime();
                        format!("{}年{}月{}日", year, month, day)
                    }else{
                        text
                    }
                };

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
