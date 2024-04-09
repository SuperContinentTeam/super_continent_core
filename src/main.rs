use assets::parse_all;
use game::world::World;
use serde::{Deserialize, Serialize};
use std::{env, fs::read_to_string};
use reference::AXState;
use state::{run_state, State};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

mod assets;
mod commander;
mod cst;
mod game;
mod player;
mod processes;
mod reference;
mod state;
mod ws;

fn main() {
    // 读取内置数据
    parse_all();
    // 启动服务
    start_server();
}

fn start_server() {
    // 读取配置
    let conf = parse_toml_config();

    let ws_addr = conf.ws_server.clone();
    let ax_state = new_state_from_meta(&conf);

    // 状态线程
    let s1 = ax_state.clone();
    let t1 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_websocket_server(ws_addr, s1));
    });

    // 通信线程
    let s2 = ax_state.clone();
    let t2 = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_state(s2));
    });

    let _ = t1.join();
    let _ = t2.join();
}

async fn start_websocket_server(addr: String, s: AXState) {
    println!("WebSocket Server Listening on: {}", addr);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(ws::handle_connection(stream, addr, s.clone()));
    }
}

fn new_state_from_meta(conf: &Configure) -> AXState {
    let s = State {
        tick: 0,
        players: HashMap::new(),
        admin: String::new(),
        max_player: conf.max_player,
        status: 0,
        world: World::new(conf.world_size),
    };

    Arc::new(Mutex::new(s))
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Configure {
    pub ws_server: String,
    pub max_player: i32,
    pub world_size: i32,
}

pub fn parse_toml_config() -> Configure {
    let args = env::args().collect::<Vec<String>>();
    let config_file = if args.len() == 1 {
        "default.toml"
    } else {
        &args[1]
    };

    let content = read_to_string(config_file).unwrap();
    let v: Configure = toml::from_str(&content).unwrap();

    v
}
