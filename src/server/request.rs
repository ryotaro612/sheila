///
use crate::command;
use crate::server::response;
use serde::{Deserialize, Serialize};
use serde_json;
use std::result;

/// Parses a JSON-RPC request string into a command and extracts the request id.
///
/// # Arguments
///
/// * `request` - The JSON-RPC request as a string.
///
/// # Returns
///
/// * `Ok((id, command))` if parsing and conversion succeed, where `id` is the request id and `command` is the parsed command.
/// * `Err(response::Response)` if there is a parsing or validation error.
pub(crate) fn parse_request(
    request: &str,
) -> Result<(String, command::Command), response::Response> {
    let parsed: serde_json::Value =
        serde_json::from_str(request).map_err(|e| response::Response::ParseError { error: e })?;
    let json_rpc_request: JsonRpcRequest = serde_json::from_value(parsed)
        .map_err(|error| response::Response::InvalidRequest { error })?;

    let command = make_command(&json_rpc_request)?;
    Ok((json_rpc_request.id, command))
}

///
fn make_command(r: &JsonRpcRequest) -> result::Result<command::Command, response::Response> {
    match r.method.as_str() {
        "stop" => r
            .as_stop_cmd()
            .map_err(|e| response::Response::InvalidParams {
                id: r.id.clone(),
                error: e,
            }),
        "status" => Ok(command::Command::Status),
        "play" => r
            .as_play_cmd()
            .map_err(|e| response::Response::InvalidParams {
                id: r.id.clone(),
                error: e,
            }),
        "shutdown" => Ok(command::Command::Shutdown),

        _ => Err(response::Response::MethodNotFound {
            id: r.id.clone(),
            error: format!("method not found: {}", r.method),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    pub(crate) id: String,
}

trait StopCommandDecoder {
    fn as_stop_cmd(&self) -> Result<command::Command, String>;
}
impl StopCommandDecoder for JsonRpcRequest {
    fn as_stop_cmd(&self) -> Result<command::Command, String> {
        let vals = self.params.as_ref().ok_or("err")?;
        let monitor = match vals["monitor"] {
            serde_json::Value::Null => Ok(None),
            serde_json::Value::String(ref s) => Ok(Some(s.clone())),
            _ => Err("monitor is not a string".to_string()),
        }?;
        Ok(command::Command::Stop { monitor })
    }
}

trait PlayCommandDecoder {
    fn as_play_cmd(&self) -> Result<command::Command, String>;
}

impl PlayCommandDecoder for JsonRpcRequest {
    fn as_play_cmd(&self) -> Result<command::Command, String> {
        let params = self.params.as_ref().ok_or("params is required")?;
        let files_ary = params
            .get("files")
            .ok_or("files is required.")?
            .as_array()
            .ok_or("files is not an array")?;

        let mut files: Vec<String> = vec![];
        for file in files_ary {
            let filepath = file.as_str().ok_or(format!("{} is not a string.", file))?;
            files.push(filepath.to_string());
        }
        if files.is_empty() {
            return Err("files is empty.".to_string());
        }
        let monitor = match params.get("monitor") {
            Some(serde_json::Value::String(s)) => Some(s.to_string()),
            _ => None,
        };
        let random = match params.get("random") {
            Some(serde_json::Value::Bool(b)) => *b,
            _ => false,
        };
        Ok(command::Command::Play {
            files,
            monitor,
            random,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_rpc_request_can_omit_params() {
        let result = serde_json::from_str::<JsonRpcRequest>(
            r#"{"jsonrpc":"2.0","method":"status","id":"id"}"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn play_requires_at_least_one_file() {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "play",
            "params": {
                "files": [],
                "monitor": Option::<String>::None,
            },
            "id": "id".to_string(),
        });
        let actual = parse_request(&request.to_string()).unwrap_err();

        if let response::Response::InvalidParams { id, error } = actual {
            assert_eq!("id", id);
            assert_eq!("files is empty.".to_string(), error);
        } else {
            panic!("unexpected response: {:?}", actual);
        }
    }

    #[test]
    fn test_method_stop_is_stop_command() {
        let request = serde_json::json!({
            "jsonrpc": "2.0".to_string(),
            "method": "stop".to_string(),
            "params": {
                "monitor": Option::<String>::None,
            },
            "id": "id".to_string(),
        });
        let (id, command) = parse_request(request.to_string().as_str()).unwrap();
        assert_eq!("id", id);
        assert_eq!(command::Command::Stop { monitor: None }, command);
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
            response::Response::MethodNotFound { id, error: _ } => {
                assert_eq!("id", id);
            }
            _ => {
                panic!("unexpected response: {:?}", actual);
            }
        }
    }
}
