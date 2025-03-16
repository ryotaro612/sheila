use std::fs;
use std::io::Read;
use std::io::Write;
use std::os::unix::net;
use std::path;
use std::process;
use std::{io, result};
use crate::server::handler;

impl <H: handler::Handler> Drop for Server<H> {
    fn drop(&mut self) {
        if path::Path::new(&self.socket).exists() {
            match fs::remove_file(&self.socket) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("error removing the socket file: {e}");
                }
            }
        }
    }
}

struct Response {
    is_stop_request: bool,
    response: serde_json::Value,
}

impl <H: handler::Handler> Server<H> {
    pub(crate) fn new(socket: String, handler: H) -> Self {
        return Server { socket, handler };
    }
    fn handle(&self, request: String)-> Response{
        let v  = serde_json::from_str(request.as_str());
        Response{is_stop_request: false, response: v.unwrap()}
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
                            let response = self.handle(payload);
                            s.write_all(response.response.to_string().as_bytes()).unwrap_or_else(|e| {
                                log::error!("error writing to a stream: {e}");
                            });
                            if response.is_stop_request {
                                self.shutdown(&s);
                                break;
                            }
                        }
                        Err(err) => {
                            log::error!("error reading from a stream: {err}");
                            s.write_all(serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                    "code": -32700,
                                    "message": "failed to read a request",
                                }
                            }).to_string().as_bytes()).unwrap_or_else(|e|{
                                log::error!("error writing to a stream: {e}");
                            });
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
    fn shutdown(&self,  stream: &net::UnixStream) {
                    stream.shutdown(std::net::Shutdown::Write).unwrap_or_else(|e|{
                        log::error!("error shutting down the stream: {e}");
                    });
    }

    fn bind(&self) -> result::Result<net::UnixListener, io::Error> {
        let skt = &self.socket;
        if path::Path::new(skt).exists() {
            let status = process::Command::new("lsof").args([skt]).status()?;
            if status.success() {
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

pub(crate) struct Server<H:handler::Handler>  {
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