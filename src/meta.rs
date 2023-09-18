use serde::Serialize;

pub struct RoomMeta {
    pub max_number: i32,
    pub status: i32,
}

impl RoomMeta {
    pub fn new() -> Self {
        Self {
            max_number: 10,
            status: 0,
        }
    }
}