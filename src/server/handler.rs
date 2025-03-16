use serde_json;

pub(crate) trait Handler {
    fn handle(&self, request: String) -> serde_json::Value;
}