use serde_json::Value;

pub mod resource;
pub mod world;
pub mod block;
pub mod zoning;

pub trait Dumps {
    fn dumps(&self, player: &str) -> Value;
}