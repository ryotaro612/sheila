use crate::command;
use crate::server::request::{self, makeCommand};
use crate::server::response;
use serde_json;
use std::result;
use std::sync::mpsc;

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
                                        Ok(_) => response::Response::Success { id: r.id },
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
                                Err(error) => {
                                    log::error!(
                                        "error sending command. request: {:?} error: {error}",
                                        r
                                    );
                                    response::Response::ServerError {
                                        id: r.id,
                                        error: error.to_string(),
                                    }
                                }
                            },
                            Err(resp) => resp,
                        }
                    }
                    Err(error) => response::Response::InvalidRequest { error },
                }
            }
            Err(error) => response::Response::ParseError { error },
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

#[test]
fn test_return_parse_error_if_malfored_json_was_passed() {
    let (sender, _) = mpsc::channel();
    let (_, result_receiver) = mpsc::channel::<result::Result<(), command::ErrorReason>>();

    let handler = DefaultHandler::new(&sender, &result_receiver);

    // act
    let response = handler.handle(&String::from("{"));

    // assert
    match response {
        response::Response::ParseError { error: _ } => {}
        _ => {
            panic!("unexpected response: {:?}", response);
        }
    }
}
