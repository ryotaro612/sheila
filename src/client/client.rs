use serde_json::json;
use std::io::Read;
use std::io::Write;
use std::net;
use std::result;

#[cfg(test)]
use mockall::{automock, predicate::*};
/**
 * A JSON-RPC client.
 */
#[cfg_attr(test, automock)]
pub(crate) trait Client {
    /**
     * Returns Ok(v) if the request was successful, or Err(e) if there was an error.
     * v is a JSON-RPC response.
     */
    fn send(
        &self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> result::Result<serde_json::Value, String>;

    /**
     */
    fn send_method(&self, id: String, method: &str) -> result::Result<serde_json::Value, String>;
}

pub(crate) struct SocketClient {
    socket: String,
}

impl Client for SocketClient {
    fn send(
        &self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> result::Result<serde_json::Value, String> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id, });

        self.request(request)
    }

    fn send_method(&self, id: String, method: &str) -> result::Result<serde_json::Value, String> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "id": id, });

        self.request(request)
    }
}

impl SocketClient {
    pub(crate) fn new(socket: &String) -> Self {
        SocketClient {
            socket: socket.clone(),
        }
    }
    fn request(&self, payload: serde_json::Value) -> result::Result<serde_json::Value, String> {
        let mut stream =
            std::os::unix::net::UnixStream::connect(&self.socket).map_err(|e| e.to_string())?;
        stream
            .write_all(payload.to_string().as_bytes())
            .map_err(|e| e.to_string())?;
        stream
            .shutdown(net::Shutdown::Write)
            .map_err(|e| e.to_string())?;
        // stream.read_timeout();
        let mut message = String::new();
        stream
            .read_to_string(&mut message)
            .map_err(|e| e.to_string())?;
        let v: serde_json::Value = serde_json::from_str(&message).map_err(|e| e.to_string())?;
        log::debug!("received: {message}");
        if v["jsonrpc"] != "2.0" {
            return Err(format!("the response is not a JSON-RPC 2.0 response"));
        }
        if payload["id"] != v["id"] {
            return Err(format!(
                "The id of the response is not the same as the id of the request: request:{}, response: {}",
                payload["id"], v["id"]
            ));
        }
        Ok(v)
    }
}
