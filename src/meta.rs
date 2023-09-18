use std::env;
use std::env::home_dir;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use toml::Value;


#[derive(Serialize, Deserialize, Debug)]
pub struct Configure {
    pub http_server: String,
    pub ws_server: String,
}

fn open_file(file_path: &str) -> String {
    let mut file_object = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("No such file {} exception: {}", file_path, e),
    };

    let mut content = String::new();
    match file_object.read_to_string(&mut content) {
        Ok(r) => r,
        Err(e) => panic!("Error reading file: {}", e)
    };

    content
}

pub fn parse_toml_config() -> Configure {
    let args = env::args().collect::<Vec<String>>();
    let config_file = if args.len() == 1 { "default.toml" } else { &args[1] };

    let content = open_file(config_file);
    let v: Configure = toml::from_str(&content).unwrap();

    v
}