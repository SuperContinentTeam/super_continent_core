extern crate lazy_static;

use std::collections::HashMap;
use lazy_static::lazy_static;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::WebSocketStream;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use futures_util::stream::SplitSink;
use tokio::net::{TcpListener, TcpStream, TcpSocket};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
};

use crate::queue::MessageQueue;

type ArcMutex<T> = Arc<Mutex<T>>;
type WS = WebSocketStream<TcpStream>;

lazy_static! {
    static ref SENDER_MAP:ArcMutex<HashMap<SocketAddr, SplitSink<WS, Message>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    println!("New WebSocket connection: {}", peer);

    let ws_stream = accept_async(stream).await.unwrap();
    let (mut tx, mut rx) = ws_stream.split();
    // {
    //     let sender_map = SENDER_MAP.clone();
    //     let mut sender_map_clone = sender_map.lock().unwrap();
    //     sender_map_clone.insert(peer.clone(), tx);
    // }

    // A threading of receive
    while let Some(msg) = rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Get Message: {}", text);
                let msg = Message::Text(format!("{}: {}", peer.to_string(), text));
                tx.send(msg).await.unwrap();
                // {
                //     let sender_map = SENDER_MAP.clone();
                //     let mut sender_map_clone = sender_map.lock().unwrap();
                //     for (_, ws) in sender_map_clone.iter_mut() {
                //         ws.send(msg.clone()).unwrap();
                //     }
                // }

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
