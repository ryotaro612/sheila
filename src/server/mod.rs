use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::{io, result};
use std::{net, path};
pub mod server;
mod handler;

// sync=false is required playbin

/**
 *  Initialize the log system
*/
pub(crate) fn run(socket: String, args: crate::parser::ServerArgs) -> result::Result<(), String> {
    let listner = bind(socket)?;

    let mut result: Result<(), String> = Ok(());
    for stream in listner.incoming() {
        match stream {
            Ok(mut s) => {
                let mut payload = String::new();
                let req = s.read_to_string(&mut payload);
                match req {
                    Ok(_) => {
                        log::debug!("received  peyload: {payload}");
                        match handle(payload) {
                            Ok(r) => {
                                s.write_all(r.as_string().as_bytes());
                                s.shutdown(net::Shutdown::Write);
                            }
                            Err(err) => {
                                log::error!("error handling a request: {err}");
                            }
                        }
                        // s.write_all("{\"id\": \"doge\"}".as_bytes());
                        // s.shutdown(net::Shutdown::Write);
                    }
                    Err(err) => {
                        log::error!("error reading from a stream: {err}");
                    }
                }
            }
            Err(err) => {
                result = Err(err.to_string());
                break;
            }
        };
    }
    result
}

struct Response {
    is_stop: bool,
    response: serde_json::Value,
}

impl Response {
    fn as_string(&self) -> String {
        self.response.to_string()
    }
}

fn handle(request: String) -> result::Result<Response, serde_json::Value> {
    Ok(Response {
        is_stop: false,
        response: serde_json::json!({}),
    })
}

/**
 * Force to bind the socket.
 * If a file exits at the path, it will be removed.
 */
fn bind(socket: String) -> result::Result<UnixListener, String> {
    if path::Path::new(&socket).exists() {
        fs::remove_file(&socket).map_err(|e| e.to_string())?;
    }
    UnixListener::bind(socket).map_err(|e| e.to_string())
}
