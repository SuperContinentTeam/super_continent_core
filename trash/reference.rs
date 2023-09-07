use serde_json::Value;

pub fn get_string(value: &Value) -> String {
    value.as_str().unwrap().to_string()
}
