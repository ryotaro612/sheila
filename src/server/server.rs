

use crate::server::handler;
use crate::server::jsonrpc;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::os::unix::net;
use std::path;
use std::process;
use std::{io, result};

impl<H: handler::Handler> Drop for Server<H> {
    fn drop(&mut self) {
        if path::Path::new(&self.socket).exists() {
            fs::remove_file(&self.socket).unwrap_or_else(|e| {
                log::error!("error removing the socket file: {e}");
            });
        }
    }
}

impl<H: handler::Handler> Server<H> {
    pub(crate) fn new(socket: String, handler: H) -> Self {
        return Server { socket, handler };
    }
    pub(crate) fn start(&self) -> result::Result<(), io::Error> {
        let listener = self.bind()?;
        let mut result: Result<(), io::Error> = Ok(());
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut payload = String::new();
                    let req = s.read_to_string(&mut payload);
                    match req {
                        Ok(_) => {
                            log::debug!("received: {payload}");
                            let response = self.handler.handle(payload);
                            let body = response.response_as_string();
                            s.write_all(body.as_bytes()).unwrap_or_else(|e| {
                                log::error!("failed to write '{body}' to a stream: {e}");
                            });
                            if response.is_stop_request {
                                self.shutdown(&s);
                                break;
                            }
                        }
                        Err(err) => {
                            log::error!("error reading from a stream: {err}");
                            jsonrpc::write_read_error(&s, &err);
                        }
                    }
                    self.shutdown(&s);
                }
                Err(err) => {
                    log::error!("error accepting a stream: {err}");
                    result = Err(err);
                    break;
                }
            };
        }
        result
    }
    fn shutdown(&self, stream: &net::UnixStream) {
        stream
            .shutdown(std::net::Shutdown::Write)
            .unwrap_or_else(|e| {
                log::error!("error shutting down the stream: {e}");
            });
    }
    /**
     *
     */
    fn bind(&self) -> result::Result<net::UnixListener, io::Error> {
        let skt = &self.socket;
        if path::Path::new(skt).exists() {
            // avoid writing lsof results in the stdout and stderr.
            let output = process::Command::new("lsof")
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .args([skt])
                .output()?;

            let stdout = String::from_utf8(output.stdout).unwrap_or_default();
            let stderr = String::from_utf8(output.stderr).unwrap_or_default();
            log::debug!("lsof stdout: {stdout}, stdierr: {stderr}");
            if output.status.success() {
                log::error!("another process is running.");
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "another process is running.",
                ));
            } else {
                fs::remove_file(skt)?;
            }
        }
        net::UnixListener::bind(skt)
    }
}

pub(crate) struct Server<H: handler::Handler> {
    socket: String,
    handler: H,
}

// #[test]
// fn test_handle() {
//     let server = Server::new("".to_string(), handler::Handler{});
//     let response = server.handle(r#"{"jsonrpc": "2.0", "method": "stop", "id": "doge"}"#.to_string());
//     assert_eq!(response.is_stop_request, true);
//     assert_eq!(response.response, serde_json::json!({"jsonrpc": "2.0", "result": "ok", "id": "doge"}));
// }
