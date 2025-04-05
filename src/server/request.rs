/**
 *
 */
use crate::command;
use crate::server::response;
use serde::{Deserialize, Serialize};
use serde_json;
use std::option;
use std::result;
/**
 *
 */
pub(crate) fn make_command(
    r: &JsonRpcRequest,
) -> result::Result<command::Command, response::Response> {
    match r.method.as_str() {
        "stop" => Ok(command::Command::Stop),
        "status" => Ok(command::Command::Status),
        "display" => r
            .as_display_cmd()
            .map_err(|e| response::Response::InvalidParams {
                id: r.id.clone(),
                error: e,
            }),

        _ => Err(response::Response::MethodNotFound {
            id: r.id.clone(),
            error: format!("method not found: {}", r.method),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: option::Option<serde_json::Value>,
    pub(crate) id: String,
}

trait DisplayCommandPresenter {
    fn as_display_cmd(&self) -> Result<command::Command, String>;
}

impl DisplayCommandPresenter for JsonRpcRequest {
    fn as_display_cmd(&self) -> Result<command::Command, String> {
        let params = self.params.as_ref().ok_or("Missing params")?;
        let file = params
            .get("file")
            .ok_or("file is required.")?
            .as_str()
            .ok_or("file is not a string")?;
        match params.get("monitor") {
            Some(m) => {
                log::debug!("monitor: {}", m);
                let monitor = m.as_str().ok_or("monitor is not a string")?;
                Ok(command::Command::Display {
                    file: file.to_string(),
                    monitor: Some(monitor.to_string()),
                })
            }
            None => Ok(command::Command::Display {
                file: file.to_string(),
                monitor: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            response::Response::MethodNotFound { id, error: _ } => {
                assert_eq!("id", id);
            }
            _ => {
                panic!("unexpected response: {:?}", actual);
            }
        }
    }

    #[test]
    fn json_rpc_request_can_represent_display_command() {
        let r = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "display".to_string(),
            params: Some(serde_json::json!({
                "file": "image.png",
                "monitor": "eDP-1"
            })),
            id: "id".to_string(),
        };
        let actual = make_command(&r).unwrap();
        match actual {
            command::Command::Display { file, monitor } => {
                assert_eq!("image.png", file);
                assert_eq!(Some("eDP-1".to_string()), monitor);
            }
            _ => panic!("unexpected command: {:?}", actual),
        }
    }

    #[test]
    fn monitor_of_display_command_is_optional() {
        let r = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "display".to_string(),
            params: Some(serde_json::json!({
                "file": "image.png",
            })),
            id: "id".to_string(),
        };
        let actual = make_command(&r).unwrap();
        assert_eq!(
            command::Command::Display {
                file: "image.png".to_string(),
                monitor: None,
            },
            actual
        );
    }
}
