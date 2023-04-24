use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

static PEER_MAP: OnceCell<PeerMap> = OnceCell::new();

pub fn initial_peer_map() {
    let result = PeerMap::new(Mutex::new(HashMap::new()));
    PEER_MAP.set(result).unwrap();
}

pub fn parse_text_message(text: String, peer: &SocketAddr) {
    let peer_map = PEER_MAP.get().unwrap().clone();
    println!("Received a message from {}: {}", peer, text);
    let message = Message::Text(format!("{}: {}", &peer, text));

    let peer_map_guard = peer_map.lock().unwrap();
    let clients = peer_map_guard.iter().map(|(addr, ws_sink)| (addr, ws_sink));
    for (addr, client) in clients {
        println!("Send Message to: {}", addr);
        client.unbounded_send(message.clone()).unwrap();
    }
}

pub async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let peer_map = PEER_MAP.get().unwrap().clone();

    println!("New WebSocket connection: {}", peer);
    let ws_stream = accept_async(stream).await.unwrap();
    let (tx, rx) = unbounded();

    peer_map.lock().unwrap().insert(peer.clone(), tx);

    let (outgoing, incoming) = ws_stream.split();

    let incoming_future = incoming.try_for_each(|msg| {
        match msg {
            Message::Text(text) => parse_text_message(text, &peer),
            Message::Binary(_) => todo!(),
            Message::Ping(_) => todo!(),
            Message::Pong(_) => todo!(),
            Message::Close(_) => {
                println!("{} disconnected", &peer);
                PEER_MAP.get().unwrap().lock().unwrap().remove(&peer);
            },
            Message::Frame(_) => todo!(),
        }

        future::ok(())
    });

    let outgoing_future = rx.map(Ok).forward(outgoing);

    pin_mut!(incoming_future, outgoing_future);
    future::select(incoming_future, outgoing_future).await;

    Ok(())
}

pub async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}
