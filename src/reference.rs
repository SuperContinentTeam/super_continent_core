use crate::state::State;
use futures_channel::mpsc::UnboundedSender;
use lazy_static::lazy_static;
use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

pub type Tx = UnboundedSender<Message>;

pub struct Client {
    pub tx: Tx,
    pub addr: SocketAddr,
    pub player: String,
}

pub type AxClient = Arc<Mutex<Client>>;
pub type AXState = Arc<Mutex<State>>;

lazy_static! {
    // 时间刻
    pub static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
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

pub fn random_between(a:i32, b:i32) -> i32 {
    rand::thread_rng().gen_range(a..b)
}
