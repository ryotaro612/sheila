use serde_json;

pub(crate) trait Handler {
    fn handle(&self, request: String) -> serde_json::Value;
}

impl Handler for DefaultHandler {
    fn handle(&self, request: String) -> serde_json::Value {
        serde_json::json!({})
    }
}

 impl DefaultHandler {
    pub(crate) fn new() -> Self {
        DefaultHandler {}
    }
}

 pub(crate) struct DefaultHandler {

}