use serde::{Deserialize, Serialize};
use std::env;

use crate::reference::read_file;

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

    let content = read_file(config_file);
    let v: Configure = toml::from_str(&content).unwrap();

    v
}
