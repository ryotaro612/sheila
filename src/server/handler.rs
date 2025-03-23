use crate::server::request::{self, makeCommand};
use crate::server::response;
use serde_json;
use std::result;
use std::sync::mpsc;

use crate::command;

pub(crate) trait Handler {
    fn handle(&self, request: &String) -> response::Response;
}

impl<'a> Handler for DefaultHandler<'a> {
    fn handle(&self, request: &String) -> response::Response {
        let v = request.as_str();
        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(v);
        match parsed {
            Ok(v) => {
                let req: serde_json::Result<request::JsonRpcRequest> = serde_json::from_value(v);
                match req {
                    Ok(r) => {
                        log::debug!("received: {:?}", r);
                        match makeCommand(&r) {
                            Ok(c) => match self.command_sender.send(c) {
                                Ok(_) => match self.result_receiver.recv() {
                                    Ok(res) => match res {
                                        Ok(_) => {
                                            log::debug!("command executed successfully");
                                            response::Response::Success { id: r.id }
                                        }
                                        Err(e) => {
                                            log::debug!("command failed: {:?}", e);
                                            match e {
                                                command::ErrorReason::InvalidParams { reason } => {
                                                    response::Response::InvalidParams {
                                                        id: r.id,
                                                        error: reason,
                                                    }
                                                }
                                                command::ErrorReason::ServerError { reason } => {
                                                    response::Response::ServerError {
                                                        id: r.id,
                                                        error: reason,
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        log::debug!("error receiving result: {e}");
                                        response::Response::ServerError {
                                            id: r.id,
                                            error: e.to_string(),
                                        }
                                    }
                                },
                                Err(e) => {
                                    log::debug!("error sending command: {e}");
                                    response::Response::ServerError {
                                        id: r.id,
                                        error: e.to_string(),
                                    }
                                }
                            },
                            Err(resp) => resp,
                        }
                    }
                    Err(e) => {
                        log::debug!("invalid request: error: {e}");
                        response::Response::InvalidRequest { error: e }
                    }
                }
            }
            Err(e) => {
                log::debug!("invalid json: {e}");
                response::Response::ParseError {
                    error: e.to_string(),
                }
            }
        }
    }
}

impl<'a> DefaultHandler<'a> {
    pub(crate) fn new(
        command_sender: &'a mpsc::Sender<command::Command>,
        result_receiver: &'a mpsc::Receiver<result::Result<(), command::ErrorReason>>,
    ) -> Self {
        DefaultHandler {
            command_sender,
            result_receiver,
        }
    }
}

pub(crate) struct DefaultHandler<'a> {
    command_sender: &'a mpsc::Sender<crate::command::Command>,
    result_receiver: &'a mpsc::Receiver<result::Result<(), command::ErrorReason>>,
}

// #[test]
// fn test_if_a_request_is_not_json_object_code_is_minus_32700() {
//     let (sender, _) = mpsc::channel();
//     let (_, receiver) = mpsc::channel();
//     let actual = DefaultHandler::new(&sender, &receiver).handle(&"".to_string());
//     assert_eq!(false, actual.is_stop_request);
//     assert_eq!("2.0", actual.response["jsonrpc"]);
//     assert_eq!(-32700, actual.response["error"]["code"]);
// }
// #[test]
// fn test_params_can_be_omitted() {
//     let a = serde_json::json!({
//         "jsonrpc": "2.0",
//         "method": "display",
//         "id": "1",
//     });
//     let actual = DefaultHandler::new().handle(&a.to_string());
//     assert_eq!(false, actual.is_stop_request);
//     assert_eq!(serde_json::Value::Null, actual.response["code"]);
// }
