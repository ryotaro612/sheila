use crate::command;
use crate::server::request::parse_request;
use crate::server::response;
use serde_json;
use std::sync::mpsc;
///
pub(crate) trait Handler {
    fn handle(&self, request: &str) -> response::Response;
}
///
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
        result_receiver: &'a mpsc::Receiver<Result<serde_json::Value, command::ErrorReason>>,
    ) -> Self {
        DefaultHandler {
            command_sender,
            result_receiver,
        }
    }
    /// Passes a recived command to the GUI handler.
    fn process(&self, payload: &str) -> Result<response::Response, response::Response> {
        let (id, command) = parse_request(&payload)?;

        self.command_sender.send(command.clone()).map_err(|error| {
            log::error!(
                "error sending command. command: {:?} error: {error}",
                command
            );
            response::Response::ServerError {
                id: id.clone(),
                error: error.to_string(),
            }
        })?;

        // Uses the Ok value as the value of the result field.
        let response = self.result_receiver.recv().map_err(|error| {
            log::debug!(
                "error receiving result: command: {:?}, error: {error}",
                command
            );
            response::Response::ServerError {
                id: id.clone(),
                error: error.to_string(),
            }
        })?;

        match response {
            Err(err_reason) => match err_reason {
                command::ErrorReason::InvalidParams { reason } => {
                    Err(response::Response::InvalidParams {
                        id: id.clone(),
                        error: reason,
                    })
                }
                command::ErrorReason::ServerError { reason } => {
                    Err(response::Response::ServerError { id, error: reason })
                }
            },
            Ok(v) => Ok(response::Response::Success { id, result: v }),
        }
    }
}

pub(crate) struct DefaultHandler<'a> {
    command_sender: &'a mpsc::Sender<crate::command::Command>,
    result_receiver: &'a mpsc::Receiver<Result<serde_json::Value, command::ErrorReason>>,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn malformed_json_makes_parse_error() {
        let (sender, _) = mpsc::channel();
        let (_, result_receiver) =
            mpsc::channel::<Result<serde_json::Value, command::ErrorReason>>();

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
}
