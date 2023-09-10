use std::str::FromStr;

use fastwebsockets::Frame;
use fastwebsockets::Payload;
use fastwebsockets::upgrade;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use tokio::net::TcpListener;

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

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);

    loop {
        let frame = ws.read_frame().await?;
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                let value = parse_to_value(&frame.payload);
                println!("{:#?}", value);

                let resp = serde_json::json!({
                    "op": "hey",
                    "payloady": value
                }).to_string();
                let resp_clone = resp.clone();
                let resp_vec_u8 = resp_clone.as_bytes();
                let payload = Payload::from(resp_vec_u8);
                let new_frame = Frame::new(true, frame.opcode, None, payload);
                ws.write_frame(new_frame).await?;
            }
            _ => {}
        }
    }

    Ok(())
}


fn parse_to_value(payload: &Payload) -> serde_json:: Value {
    let u8_content  = payload.to_vec();
    let content = std::str::from_utf8(&u8_content).unwrap();
    serde_json::Value::from_str(content).unwrap()
}