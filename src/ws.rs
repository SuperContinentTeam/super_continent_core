use crate::{
    commander,
    reference::{AXState, AxClient, Client},
};
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, s: AXState) {
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
        player: String::new(),
    };
    let ax_client = Arc::new(Mutex::new(client));
    let client_clone = ax_client.clone();

    // outgoing 用于发送数据到 WebSocket 连接，incoming 用于接收从 WebSocket 连接接收到的数据
    let (outgoing, incoming) = ws_stream.split();
    let s1 = s.clone();
    let receive_from = incoming.try_for_each(move |msg| {
        tokio::task::spawn(process_message_from_client(
            msg,
            ax_client.clone(),
            s1.clone(),
        ));
        future::ok(())
    });

    // 使用tx发送数据, rx会收到数据, 然后把数据流转发给outgoing, 从而实现发送消息的功能
    let send_from = rx.map(Ok).forward(outgoing);
    pin_mut!(receive_from, send_from);
    future::select(receive_from, send_from).await;
    println!("{} disconnected", &addr);

    tokio::task::spawn(close_websocket(client_clone, s.clone()));

}

pub async fn process_message_from_client(msg: Message, client: AxClient, s: AXState) {
    match msg {
        Message::Binary(msg) => {
            commander::bypass_binary(
                std::str::from_utf8(&msg).unwrap(),
                client.clone(),
                s.clone(),
            )
            .await
        }
        Message::Ping(v) => {
            let _ = client.lock().await.tx.unbounded_send(Message::Pong(v));
        }
        _ => {}
    }
}

pub async fn send_message(msg: String, tx: AxClient) {
    let m = msg.as_bytes().to_vec();
    let _ = tx.lock().await.tx.unbounded_send(Message::Binary(m));
}

async fn close_websocket(client: AxClient, state: AXState) {
    let mut s = state.lock().await;
    let c = client.lock().await;

    s.remove_player(&c.player);
}
