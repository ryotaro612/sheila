use std::{
    io::Error,
    io::Write,
    os::unix::net::{self},
};
use serde_json;

pub(crate) fn write_read_error(mut stream: &net::UnixStream, _error: &Error) {
    stream
        .write_all(
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32700,
                    "message": "failed to read a request",
                }
            })
            .to_string()
            .as_bytes(),
        )
        .unwrap_or_else(|e| {
            log::error!("error writing to a stream: {e}");
        });
}

pub(crate) fn stop_command() -> String {
    serde_json::json!({
        "jsonrpc": "2.0",
        "method": "stop",
        "id": "f9443da7-05d2-419d-9b86-4d5d86641d08",
    }).to_string()
}