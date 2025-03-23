use std::{io::Write, os::unix::net};

impl Response {
    /**
     * TODO errorの構造が誤っている. JSONrpcの私用でない
     */
    fn response_as_string(&self) -> String {
        match self {
            Response::Success { id } => serde_json::json!({
                "jsonrpc": "2.0",
                "result": "success",
                "id": id,
            })
            .to_string(),
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
            // FIXME
            Response::ServerError { id, error } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": error,
                    "id": id,
                })
                .to_string();
            }

            // FIXME
            Response::InvalidParams { id, error } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": error,
                    "id": id,
                })
                .to_string();
            }
            // FIXME
            Response::InternalError { error } => {
                return serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": error,
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

pub(crate) fn new_parse_error(error: serde_json::Error) -> Response {
    return Response::ParseError { error };
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
    InvalidParams { id: String, error: String },
    InternalError { error: String },
    ServerError { id: String, error: String },
}

#[test]
fn test_parse_erro_code_is_minus_32700() {
    let c = serde_json::from_str::<serde_json::Value>("\"");

    let sut = Response::ParseError {
        error: c.unwrap_err(),
    };

    let actual = sut.response_as_string();

    let v: serde_json::Value = serde_json::from_str(actual.as_str()).unwrap();

    assert_eq!("2.0", v["jsonrpc"]);
    assert_eq!(-32700, v["error"]["code"]);
}
