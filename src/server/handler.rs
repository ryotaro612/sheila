use crate::command;
use crate::server::request::{self, make_command};
use crate::server::response;
use serde_json;
use std::sync::mpsc;

pub(crate) trait Handler {
    fn handle(&self, request: &str) -> response::Response;
}

impl<'a> Handler for DefaultHandler<'a> {
    fn handle(&self, request: &str) -> response::Response {
        match self.process(request) {
            Ok(r) => r,
            Err(e) => e,
        }
    }
}

impl<'a> DefaultHandler<'a> {
    pub(crate) fn new(
        command_sender: &'a mpsc::Sender<command::Command>,
        result_receiver: &'a mpsc::Receiver<Option<command::ErrorReason>>,
    ) -> Self {
        DefaultHandler {
            command_sender,
            result_receiver,
        }
    }

    fn process(&self, payload: &str) -> Result<response::Response, response::Response> {
        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(payload);
        let json_value = parsed.map_err(|error| response::Response::ParseError { error })?;
        let json_rpc_request: request::JsonRpcRequest = serde_json::from_value(json_value)
            .map_err(|error| response::Response::InvalidRequest { error })?;
        let command = make_command(&json_rpc_request)?;
        log::debug!("sending a command: {:?}", command);
        self.command_sender.send(command).map_err(|error| {
            log::error!(
                "error sending command. request: {:?} error: {error}",
                json_rpc_request,
            );
            response::Response::ServerError {
                id: json_rpc_request.id.clone(),
                error: error.to_string(),
            }
        })?;
        let response = self.result_receiver.recv().map_err(|error| {
            log::debug!(
                "error receiving result: request: {:?}, error: {error}",
                &json_rpc_request
            );
            response::Response::ServerError {
                id: json_rpc_request.id.clone(),
                error: error.to_string(),
            }
        })?;

        match response {
            Some(err_reason) => match err_reason {
                command::ErrorReason::InvalidParams { reason } => {
                    Err(response::Response::InvalidParams {
                        id: json_rpc_request.id.clone(),
                        error: reason,
                    })
                }
                command::ErrorReason::ServerError { reason } => {
                    Err(response::Response::ServerError {
                        id: json_rpc_request.id.clone(),
                        error: reason,
                    })
                }
            },
            None => Ok(response::Response::Success {
                id: json_rpc_request.id.clone(),
            }),
        }
    }
}

pub(crate) struct DefaultHandler<'a> {
    command_sender: &'a mpsc::Sender<crate::command::Command>,
    result_receiver: &'a mpsc::Receiver<Option<command::ErrorReason>>,
}

#[test]
fn test_return_parse_error_if_malfored_json_was_passed() {
    let (sender, _) = mpsc::channel();
    let (_, result_receiver) = mpsc::channel::<Option<command::ErrorReason>>();

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
