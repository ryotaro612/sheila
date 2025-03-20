use std::{
    io::Error,
    io::Write,
    os::unix::net::{self, UnixStream},
};



pub(crate) fn write_read_error(mut stream: &net::UnixStream, error: &Error) {
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
