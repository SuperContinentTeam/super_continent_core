pub mod state;
pub mod manager;
pub mod resource;

pub trait NextState {
    fn next(&mut self);
}