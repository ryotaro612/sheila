use std::result;
mod handler;
mod jsonrpc;
mod server;

// sync=false is required playbin

/**
 *  Initialize the log system
*/
pub(crate) fn run(socket: String) -> result::Result<(), String> {
    let server = server::Server::new(socket, handler::DefaultHandler::new());
    server.start().map_err(|e| e.to_string())
}

// struct Response {
//     is_stop: bool,
//     response: serde_json::Value,
// }

// impl Response {
//     fn as_string(&self) -> String {
//         self.response.to_string()
//     }
// }

// fn handle(request: String) -> result::Result<Response, serde_json::Value> {
//     Ok(Response {
//         is_stop: false,
//         response: serde_json::json!({}),
//     })
// }

// /**
//  * Force to bind the socket.
//  * If a file exits at the path, it will be removed.
//  */
// fn bind(socket: String) -> result::Result<UnixListener, String> {
//     if path::Path::new(&socket).exists() {
//         fs::remove_file(&socket).map_err(|e| e.to_string())?;
//     }
//     UnixListener::bind(socket).map_err(|e| e.to_string())
// }
