use fastwebsockets::{upgrade, FragmentCollector, Frame, OpCode, Payload, WebSocketError};
use hyper::{server::conn::Http, service::service_fn, upgrade::Upgraded, Body, Request, Response};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

use crate::commander;

pub type AXController = Arc<Mutex<FragmentCollector<Upgraded>>>;
pub type PeerMap = Arc<Mutex<HashMap<String, HashMap<String, AXController>>>>;

lazy_static! {
    pub static ref PEER_MAP: PeerMap = PeerMap::default();
}

pub async fn start_server() {
    let bind_addr = "0.0.0.0:7000".to_string();
    println!("Start Server in: {:?}", bind_addr);
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    loop {
        let (stream, client_addr) = listener.accept().await.unwrap();
        println!("Client connected: {}", client_addr);
        tokio::spawn(async move {
            let conn_fut = Http::new()
                .serve_connection(stream, service_fn(server_upgrade))
                .with_upgrades();
            if let Err(e) = conn_fut.await {
                println!("An error occurred: {:?}", e);
            }
        });
    }
}

async fn server_upgrade(mut req: Request<Body>) -> Result<Response<Body>, WebSocketError> {
    let (response, fut) = upgrade::upgrade(&mut req)?;
    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });

    Ok(response)
}

fn parse_to_value(payload: &Payload) -> Value {
    let u8_content = payload.to_vec();
    let content = std::str::from_utf8(&u8_content).unwrap();

    Value::from_str(content).unwrap()
}

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let ws = fastwebsockets::FragmentCollector::new(fut.await?);
    let ax_ws = Arc::new(Mutex::new(ws));

    loop {
        let frame = {
            let mut ws_gurand = ax_ws.lock().await;
            ws_gurand.read_frame().await?
        };

        match frame.opcode {
            OpCode::Close => {
                println!("Client disconnected");
                break;
            }
            OpCode::Text | OpCode::Binary => {
                let value = parse_to_value(&frame.payload);
                if let Some(op) = value.get("op") {
                    let op = op.as_str().unwrap();
                    match op {
                        "join" => {
                            let result = commander::join_room(value, ax_ws.clone()).await;
                            send_message(&result, &ax_ws).await;
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn send_message(message: &Value, receiver: &AXController) {
    let receiver_clone = receiver.clone();
    let str_message = message.to_string();
    let u8_message = str_message.as_bytes();
    let payload = Payload::from(u8_message);
    let frame = Frame::new(true, OpCode::Text, None, payload);
    let mut receiver_guard = receiver_clone.lock().await;
    receiver_guard.write_frame(frame).await.unwrap();
}

async fn broadcast(room: &str, message: &Value) {
    let peer_map_clone = PEER_MAP.clone();
    let peer_map = peer_map_clone.lock().await;

    if let Some(ws_map) = peer_map.get(room) {
        for ws in ws_map.values() {
            send_message(message, ws).await;
        }
    }
}

async fn connect_server(message: Value, websocket: AXController) -> Value {
    json!({})
}

