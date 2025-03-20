use serde_json;

pub(crate) struct Response {
    pub(crate) is_stop_request: bool,
    response: serde_json::Value,
}

impl Response {
    pub(crate) fn response_as_string(&self) -> String {
        self.response.to_string()
    }
}

pub(crate) trait Handler {
    fn handle(&self, request: String) -> Response;
}

impl Handler for DefaultHandler {
    fn handle(&self, _request: String) -> Response {
        Response {
            is_stop_request: true,
            response: serde_json::json!({}),
        }
    }
}

impl DefaultHandler {
    pub(crate) fn new() -> Self {
        DefaultHandler {}
    }
}

pub(crate) struct DefaultHandler {}
