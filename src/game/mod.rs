use serde_json::Value;

pub mod resource;
pub mod world;
pub mod block;
pub mod zoning;
pub mod people;

pub trait Dumps {
    fn dumps(&self, player: &str) -> Value;
}