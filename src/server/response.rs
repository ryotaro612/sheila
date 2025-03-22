use std::{io::Write, os::unix::net};

impl Response {
    fn response_as_string(&self) -> String {
        match self {
            Response::ParseError { error: e } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                      "code": -32700,
                      "message": format!("invalid json: {}", e),
                    }
                })
                .to_string();
            }
            Response::InvalidRequest { error: e } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                      "code": -32600,
                      "message": format!("invalid request: {}", e),
                    }
                })
                .to_string();
            }
            Response::Success { id } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "result": "success",
                    "id": id,
                })
                .to_string();
            }
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
pub(crate) fn write_parse_error_response(stream: &net::UnixStream, message: &str) {
    write_response(
        &stream,
        &Response::ParseError {
            error: String::from(message),
        },
    );
}

#[derive(Debug)]
pub(crate) enum Response {
    Success { id: String },
    ParseError { error: String },
    InvalidRequest { error: serde_json::Error },
}

#[test]
fn test_parse_erro_code_is_minus_32700() {
    let sut = Response::ParseError {
        error: String::from("parse error"),
    };

    let actual = sut.response_as_string();

    let v: serde_json::Value = serde_json::from_str(actual.as_str()).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32700, v["error"]["code"]);
}
