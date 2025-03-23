use serde::{Deserialize, Serialize};
use serde_json;
use std::option;


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: option::Option<serde_json::Value>,
    pub(crate) id: String,
}

pub(crate) fn makeCommand() {

}