use crate::command;
use serde::{Deserialize, Serialize};
use serde_json;
use std::option;
use std::result;

use super::response::Response;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: option::Option<serde_json::Value>,
    pub(crate) id: String,
}

pub(crate) fn makeCommand(r: &JsonRpcRequest) -> result::Result<command::Command, Response> {
    match r.method.as_str() {
        "stop" => Ok(command::Command::Stop),
        _ => Ok(command::Command::Stop), // _ => Err(Response::MethodNotFound {
                                         //     error: format!("method not found: {r.method}"),
                                         // }),
    }
}
