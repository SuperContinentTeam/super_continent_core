use fastwebsockets::{upgrade, FragmentCollector, Frame, OpCode, Payload, WebSocketError};
use hyper::{server::conn::Http, service::service_fn, upgrade::Upgraded, Body, Request, Response};
use lazy_static::lazy_static;
use serde_json::Value;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

use crate::commander;

pub type AXController = Arc<Mutex<FragmentCollector<Upgraded>>>;

// 连接表, 键: 玩家名. 值: 连接对象
pub type ClientMap = Arc<Mutex<HashMap<String, AXController>>>;

lazy_static! {
    pub static ref CLIENT_MAP: ClientMap = ClientMap::default();
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
                    if let Some(op) = op.as_str() {
                        commander::bypass(op, value.clone(), ax_ws.clone()).await;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

pub async fn add_client(name: String, ws: AXController) {
    let client_map_clone = CLIENT_MAP.clone();
    let mut client_map = client_map_clone.lock().await;
    if !client_map.contains_key(&name) {
        client_map.insert(name, ws);
    }
}

pub async fn send_message(message: &Value, receiver: &AXController) {
    let receiver_clone = receiver.clone();
    let str_message = message.to_string();
    let u8_message = str_message.as_bytes();
    let payload = Payload::from(u8_message);
    let frame = Frame::new(true, OpCode::Text, None, payload);
    let mut receiver_guard = receiver_clone.lock().await;
    let _ = receiver_guard.write_frame(frame).await;
}

pub async fn broadcast(players: &Vec<String>, message: &Value) {
    let str_message = message.to_string();
    let u8_message = str_message.as_bytes();

    let clients: Vec<AXController> = {
        let client_map_clone = CLIENT_MAP.clone();
        let client_map = client_map_clone.lock().await;

        client_map.iter().filter_map(|(x,y)| {
            if players.contains(x) {
                Some(y.clone())
            }else {
                None
            }
        }).collect()
    };
    println!("Broadcast: {:#?}", players);
    println!("Client: {}", clients.len());
    for client in clients {
        println!("1");
        let client_clone = client.clone();
        let mut client = client_clone.lock().await;
        println!("2");

        let _ = client
            .write_frame(Frame::new(
                true,
                OpCode::Text,
                None,
                Payload::from(u8_message),
            ))
            .await;
    }
}
