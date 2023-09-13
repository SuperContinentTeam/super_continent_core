use crate::{commander, state::state::STATE_MAP};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use lazy_static::lazy_static;
use serde_json::Value;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;

pub type Tx = UnboundedSender<Message>;

pub struct Client {
    pub tx: Tx,
    pub addr: SocketAddr,
}

pub type AxClient = Arc<Mutex<Client>>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, AxClient>>>;
pub type PeerUserMap = Arc<Mutex<HashMap<String, SocketAddr>>>;

lazy_static! {
    pub static ref PEER_MAP: PeerMap = PeerMap::default();
    pub static ref PEER_USER_MAP: PeerUserMap = PeerUserMap::default();
}

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // tx 用于发送数据到数据流中，而 rx 用于从数据流中接收数据
    let (tx, rx) = unbounded();
    let client = Client {
        addr: addr.clone(),
        tx,
    };
    let ax_client = Arc::new(Mutex::new(client));

    PEER_MAP.lock().await.insert(addr, ax_client.clone());

    // outgoing 用于发送数据到 WebSocket 连接，incoming 用于接收从 WebSocket 连接接收到的数据
    let (outgoing, incoming) = ws_stream.split();
    let receive_from = incoming.try_for_each(move |msg| {
        tokio::task::spawn(process_message_from_client(msg, ax_client.clone()));
        future::ok(())
    });

    // 使用tx发送数据, rx会收到数据, 然后把数据流转发给outgoing, 从而实现发送消息的功能
    let send_from = rx.map(Ok).forward(outgoing);

    pin_mut!(receive_from, send_from);
    future::select(receive_from, send_from).await;
    println!("{} disconnected", &addr);

    PEER_MAP.lock().await.remove(&addr);
    tokio::task::spawn(close_and_stop_state(addr.clone()));
}

pub async fn process_message_from_client(msg: Message, client: AxClient) {
    match msg {
        Message::Binary(msg) => {
            commander::bypass_binary(std::str::from_utf8(&msg).unwrap(), client.clone()).await
        }
        Message::Ping(v) => {
            let _ = client.lock().await.tx.unbounded_send(Message::Pong(v));
        }
        _ => {}
    }
}

pub async fn send_message(msg: Value, tx: AxClient) {
    let str_message = msg.to_string();
    let _ = tx
        .lock()
        .await
        .tx
        .unbounded_send(Message::Text(str_message));
}

pub async fn broadcast(players: &Vec<String>, msg: &Value) {
    let clients = {
        let mut result: Vec<AxClient> = Vec::new();
        let peer_map = PEER_MAP.lock().await;
        let peer_user_map = PEER_USER_MAP.lock().await;
        for player in players {
            if let Some(addr) = peer_user_map.get(player) {
                if let Some(c) = peer_map.get(addr) {
                    result.push(c.clone());
                }
            }
        }
        result
    };

    for client in clients {
        tokio::task::spawn(send_message(msg.clone(), client));
    }
}

async fn close_and_stop_state(addr: SocketAddr) {
    let mut peer_user_map = PEER_USER_MAP.lock().await;

    let user = {
        let mut result: Option<String> = None;
        for (u, a) in peer_user_map.iter() {
            if a == &addr {
                result = Some(u.to_string());
                break;
            }
        }
        result
    };
    if let Some(u) = user {
        peer_user_map.remove(&u);

        let mut state_map = STATE_MAP.lock().await;
        if let Some(ax_s) = state_map.get(&u) {
            let mut s = ax_s.lock().await;
            if s.players.len() == 0 {
                s.status = 2;
            }
        }
        state_map.remove(&u);
    }
}
