use lazy_static::lazy_static;
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::{sync::Arc, collections::HashMap};

use tokio::sync::{ RwLock, mpsc};
use warp::filters::ws::{WebSocket, Message};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
type Clients = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;

lazy_static! {
    static ref PEER_MAP: Clients = Clients::default();
}

pub async fn ws_handler(ws: WebSocket) {
    let(mut client_ws_tx, mut client_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            client_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });
}