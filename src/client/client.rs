use serde_json::json;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net;
use std::result;
/**
 * A JSON-RPC client.
 */
pub(crate) trait Client {
    /**
     * 
     */
    fn send(
        &self,
        id: String,
        method: &str,
        params: serde_json::Value,
    ) -> result::Result<serde_json::Value, io::Error>;

    /**
     */
    fn send_method(&self, id: String, method: &str)
        -> result::Result<serde_json::Value, io::Error>;
}

pub(crate) struct SocketClient {
    socket: String,
}

impl Client for SocketClient {
    fn send(
        &self,
        id: String,
        method: &str,
        params: serde_json::Value,
    ) -> result::Result<serde_json::Value, io::Error> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id, });

        self.request(request)
    }

    fn send_method(
        &self,
        id: String,
        method: &str,
    ) -> result::Result<serde_json::Value, io::Error> {
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
    fn request(&self, payload: serde_json::Value) -> result::Result<serde_json::Value, io::Error> {
        let mut stream = std::os::unix::net::UnixStream::connect(&self.socket)?;
        stream.write_all(payload.to_string().as_bytes())?;
        stream.shutdown(net::Shutdown::Write)?;
        // stream.read_timeout();
        let mut message = String::new();
        stream.read_to_string(&mut message)?;
        let v: serde_json::Value = serde_json::from_str(&message)?;
        log::debug!("received: {message}");
        Ok(v)
    }
}
