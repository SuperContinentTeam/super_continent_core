use std::net::SocketAddr;

use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Result, protocol::Message},
};

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:9002".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => ws_stream.send(Message::Text(format!("You said: {}", text))).await?,
            Ok(Message::Close(_)) => {
                println!("Client disconnect");
                break;
            },
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
