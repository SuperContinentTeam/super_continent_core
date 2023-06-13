#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub body: serde_json::Value,
}

impl Message {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "sender": self.sender.clone(),
            "receiver": self.receiver.clone(),
            "body": self.body.clone()
        })
    }
}
