use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::{Error, Message}, WebSocketStream};
use tokio::net::{TcpListener, TcpStream};
use once_cell::sync::OnceCell;
use futures_util::{SinkExt, StreamExt};
// use crate::game;
// use crate::state::state::get_game_state;

// #[derive(Debug)]
// pub struct PlayerConnect {
//     pub addr: SocketAddr,
//     pub websocket: WebSocketStream<TcpStream>,
// }

type MutexPeer = Mutex<HashMap<SocketAddr, Arc<WebSocketStream<TcpStream>>>>;

static PEER_MAP: OnceCell<MutexPeer> = OnceCell::new();

pub fn initial_peer_map() {
    PEER_MAP.set(MutexPeer::new(HashMap::new())).unwrap();
}

pub async fn once_connect(peer: SocketAddr, tcp_stream: TcpStream) -> Arc<WebSocketStream<TcpStream>> {
    let peer_map = PEER_MAP.get().unwrap();
    match peer_map.lock().unwrap().get(&peer) {
        None => {
            let result = Arc::new(accept_async(tcp_stream).await.unwrap());
            peer_map.lock().unwrap().insert(peer, result.clone());
            result.clone()
        }
        Some(ws) => ws.clone()
    }
}

pub async fn connect_from_client(listener: TcpListener) {
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, stream));
    }
}


pub async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}


pub async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let peer_map = PEER_MAP.get().unwrap();
    let mut ws_stream = once_connect(peer, stream).await;
    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Get Message: {}", text);

                let mut guard = peer_map.lock().unwrap();
                for (addr, ws) in guard.iter_mut() {
                    let mut member = ws.clone();
                    let message = Message::Text(format!("{}: {}", addr, text));
                    if member.send(message).await.is_err() {
                        eprintln!("Error sending message to client")
                    }
                }

                // let message = game::command_executor(get_game_state(), &text).await;
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
