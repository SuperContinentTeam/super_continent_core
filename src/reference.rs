use futures_channel::mpsc::UnboundedSender;
use lazy_static::lazy_static;
use rand::{distributions::WeightedIndex, prelude::Distribution};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
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
    // WebSocket 发送数据的通道表
    pub static ref PEER_MAP: PeerMap = PeerMap::default();
    // 用户与SocketAddr对应表
    pub static ref PEER_USER_MAP: PeerUserMap = PeerUserMap::default();
    
    // 地块环境表 [死寂、恶劣、一般、良好、理想]
    pub static ref ENVIRONMENT_TYPES: [i32; 5] = [-2, -1, 0, 1, 2];
    // 生成各地块环境的随机权重
    pub static ref WI: WeightedIndex<i32> = WeightedIndex::new([6, 20, 51, 15, 4].iter()).unwrap();
}

pub fn random_block_env() -> i32 {
    let mut rng = rand::thread_rng();
    let v = ENVIRONMENT_TYPES[WI.sample(&mut rng)];
    v
}
