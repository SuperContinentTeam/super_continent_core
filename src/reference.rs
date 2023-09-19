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
    // 参数配置
    pub static ref STATE_CONFIG: Arc<Mutex<serde_json::Value>> = Arc::new(Mutex::new(serde_json::Value::Null));
    // 时间刻
    pub static ref TIME_FLOW: tokio::time::Duration = tokio::time::Duration::from_secs(1);
    // 地块环境表 [死寂、恶劣、一般、良好、理想]
    pub static ref ENVIRONMENT_TYPES: [i32; 5] = [-2, -1, 0, 1, 2];
    // 生成各地块环境的随机权重
    pub static ref WI: WeightedIndex<i32> = WeightedIndex::new([6, 20, 51, 15, 4].iter()).unwrap();
    // 地块产物表 [能量、矿物、食物]
    pub static ref BLOCK_PRODUCT: [String; 4] = ["e".to_string(), "m".to_string(), "f".to_string(), String::new()];
    // 产物权重表
    pub static ref PI: WeightedIndex<i32> = WeightedIndex::new([3, 4, 5, 6].iter()).unwrap();
}

pub fn random_block_env() -> i32 {
    let mut rng = rand::thread_rng();
    let v = ENVIRONMENT_TYPES[WI.sample(&mut rng)];
    v
}

pub fn random_between(a: i32, b: i32) -> i32 {
    rand::thread_rng().gen_range(a..b)
}

pub fn random_product(ev: i32) -> (i32, i32, i32) {
    println!("in random product");
    let mut e = 0;
    let mut m = 0;
    let mut f = 0;

    for _ in 0..BLOCK_PRODUCT.len() {
        let mut rng = rand::thread_rng();
        let p = random_between(0, 5);
        match BLOCK_PRODUCT[PI.sample(&mut rng)].as_str() {
            "e" => e += p,
            "m" => m += p,
            "f" => f += p,
            _ => {}
        }
    }

    let modifier = match ev {
        -2 => 0.0,
        -1 => 0.2,
        0 => 1.0,
        1 => 1.2,
        2 => 1.5,
        _ => 1.0,
    };

    let fe = (e as f64) * modifier;
    let fm = (m as f64) * modifier;
    let ff = (f as f64) * modifier;

    (fe as i32, fm as i32, ff as i32)
}
