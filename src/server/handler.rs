use serde::{Deserialize, Serialize};
use serde_json;
use std::option;

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: option::Option<serde_json::Value>,
    id: String,
}

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
    fn handle(&self, request: &String) -> Response;
}

impl Handler for DefaultHandler {
    fn handle(&self, request: &String) -> Response {
        let v = request.as_str();
        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(v);
        match parsed {
            Ok(v) => {
                let req: serde_json::Result<JsonRpcRequest> = serde_json::from_value(v);
                match req {
                    Ok(r) => {
                        log::debug!("received: {:?}", r);
                        return Response {
                            is_stop_request: false,
                            response: serde_json::json!({}),
                        };
                    }
                    Err(e) => {
                        log::debug!("invalid request: error: {e}");
                        return Response {
                            is_stop_request: false,
                            response: serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                "code": -32600,
                                "message": format!("invalid request: {}", e),
                                }

                            }),
                        };
                    }
                }
            }
            Err(e) => {
                log::debug!("invalid json: {e}");
                return Response {
                    is_stop_request: false,
                    response: serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": {
                          "code": -32700,
                          "message": format!("invalid json: {}", e),
                        }

                    }),
                };
            }
        }
    }
}

impl DefaultHandler {
    pub(crate) fn new() -> Self {
        DefaultHandler {}
    }
}

pub(crate) struct DefaultHandler {}

#[test]
fn test_if_a_request_is_not_json_object_code_is_minus_32700() {
    let actual = DefaultHandler::new().handle(&"".to_string());
    assert_eq!(false, actual.is_stop_request);
    assert_eq!("2.0", actual.response["jsonrpc"]);
    assert_eq!(-32700, actual.response["error"]["code"]);
}
// #[test]
// fn test_params_can_be_omitted() {
//     let a = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "display",
//         "id": "1",
//     });
//     let actual = DefaultHandler::new().handle(&a.to_string());
//     assert_eq!(false, actual.is_stop_request);
//     assert_eq!(serde_json::Value::Null, actual.response["code"]);
// }
