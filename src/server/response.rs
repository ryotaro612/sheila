/**
 * JSON RPC response
 */
use std::{io::Write, os::unix::net};

impl Response {
    fn response_as_string(&self) -> String {
        match self {
            Response::Success { id } => serde_json::json!({
                "jsonrpc": "2.0",
                "result": "success",
                "id": id,
            })
            .to_string(),
            Response::MethodNotFound { error, id } => new_err_response(&-32601, error, &Some(id)),

            Response::ParseError { error } => new_err_response(&-32700, &error.to_string(), &None),
            Response::InvalidRequest { error: e } => {
                new_err_response(&-32600, &format!("{}", e), &None)
            }
            Response::ServerError { id, error } => {
                new_err_response(&-32000, error, &Some(id.as_str()))
            }
            Response::InvalidParams { id, error } => {
                new_err_response(&-32602, error, &Some(id.as_str()))
            }
            Response::InternalError { error } => new_err_response(&-32603, error, &None),
        }
    }
    pub(crate) fn is_stop_request(&self) -> bool {
        match self {
            Response::Success { id: _ } => true,
            _ => false,
        }
    }
}

/**
 *
 */
pub(crate) fn write_response(mut stream: &net::UnixStream, response: &Response) {
    stream
        .write_all(response.response_as_string().as_bytes())
        .unwrap_or_else(|e| {
            log::error!("error writing {:?} to a stream: {e}", response);
        });
}

#[derive(Debug)]
pub(crate) enum Response {
    Success { id: String },
    ParseError { error: serde_json::Error },
    InvalidRequest { error: serde_json::Error },
    MethodNotFound { id: String, error: String },
    InvalidParams { id: String, error: String },
    InternalError { error: String },
    ServerError { id: String, error: String },
}

fn new_err_response(code: &i32, message: &str, id: &Option<&str>) -> String {
    let mut response = serde_json::json!({
        "jsonrpc": "2.0",
        "error": {
            "code": code,
            "message": message,
        }
    });
    if let Some(id) = id {
        response["id"] = serde_json::json!(id);
    }
    response.to_string()
}

#[test]
fn test_response_as_string_success() {
    let response = Response::Success {
        id: "123".to_string(),
    };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!("success", v["result"]);
    assert_eq!("123", v["id"]);
}
#[test]
fn test_parse_error_meets_jsonrpc2_spec() {
    let error = serde_json::from_str::<serde_json::Value>("\"").unwrap_err();
    let response = Response::ParseError { error };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32700, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
}
#[test]
fn test_invalid_request_meets_jsonrpc2_spec() {
    let error = serde_json::from_str::<serde_json::Value>("\"").unwrap_err();
    let response = Response::InvalidRequest { error };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32600, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
}
#[test]
fn test_server_error_meets_jsonrpc2_spec() {
    let response = Response::ServerError {
        id: "456".to_string(),
        error: "server error occurred".to_string(),
    };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32000, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
    assert_eq!("456", v["id"]);
}
#[test]
fn test_invalid_params_meets_jsonrpc2_spec() {
    let response = Response::InvalidParams {
        id: "789".to_string(),
        error: "invalid parameters".to_string(),
    };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32602, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
    assert_eq!("789", v["id"]);
}
#[test]
fn test_internal_error_meets_jsonrpc2_spec() {
    let response = Response::InternalError {
        error: "internal error occurred".to_string(),
    };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32603, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
}
#[test]
fn test_method_not_found_meets_jsonrpc2_spec() {
    let response = Response::MethodNotFound {
        id: "foobar".to_string(),
        error: "a".to_string(),
    };
    let actual = response.response_as_string();
    let v: serde_json::Value = serde_json::from_str(&actual).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32601, v["error"]["code"]);
    assert!(v["error"]["message"].is_string());
}
