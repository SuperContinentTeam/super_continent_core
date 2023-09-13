use crate::commander;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use lazy_static::lazy_static;
use serde_json::Value;
use std::{collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;

pub type Tx = UnboundedSender<Message>;

pub struct Client {
    pub tx: Tx,
    pub addr: SocketAddr
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
        tx
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
}

pub async fn process_message_from_client(msg: Message, client: AxClient) {
    if let Ok(message) = parse_to_value(&msg) {
        if let Some(op) = message.get("op") {
            if let Some(str_op) = op.as_str() {
                commander::bypass(str_op, message.clone(), client.clone()).await
            }
        }
    }
}

fn parse_to_value(message: &Message) -> Result<Value, serde_json::Error> {
    let u8_content = message.to_text().unwrap().as_bytes();
    let content = std::str::from_utf8(&u8_content).unwrap();

    Value::from_str(content)
}

pub async fn send_message(msg: &Value, tx: AxClient) {
    let str_message = msg.to_string();
    let _ = tx.lock().await.tx.unbounded_send(Message::Text(str_message));
}