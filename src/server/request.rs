use crate::command;
use crate::server::response;
use crate::server::response::Response;
use serde::{Deserialize, Serialize};
use serde_json;
use std::option;
use std::result;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: option::Option<serde_json::Value>,
    pub(crate) id: String,
}

pub(crate) fn make_command(
    r: &JsonRpcRequest,
) -> result::Result<command::Command, response::Response> {
    match r.method.as_str() {
        "stop" => Ok(command::Command::Stop),
        _ => Err(response::Response::MethodNotFound {
            id: r.id.clone(),
            error: format!("method not found: {}", r.method),
        }),
    }
}

#[test]
fn test_method_stop_is_stop_command() {
    let r = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "stop".to_string(),
        params: None,
        id: "id".to_string(),
    };
    let actual = make_command(&r).unwrap();
    assert_eq!(command::Command::Stop, actual);
}

#[test]
fn test_method_not_found_unknown_is_returned_if_method_is_unknown() {
    let r = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "foobar".to_string(),
        params: None,
        id: "id".to_string(),
    };
    let actual = make_command(&r).unwrap_err();
    match actual {
        Response::MethodNotFound { id, error: _ } => {
            assert_eq!("id", id);
        }
        _ => {
            panic!("unexpected response: {:?}", actual);
        }
    }
}
