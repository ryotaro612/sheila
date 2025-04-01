use std::result;
mod handler;
mod request;
mod response;
mod server;
use crate::command;
use crate::draw;
use std::sync::mpsc;
use std::thread;

// sync=false is required playbin

/**
 *  Initializes the log system.
*/
pub(crate) fn run(socket: String) -> result::Result<(), String> {
    let (command_sender, command_receiver) = mpsc::channel::<command::Command>();
    let (result_sender, result_receiver) = mpsc::channel::<Option<command::ErrorReason>>();

    let server_join = thread::spawn(move || {
        let server = server::Server::new(
            socket,
            handler::DefaultHandler::new(&command_sender, &result_receiver),
        );
        server.start().map_err(|e| e.to_string())
    });

    let drawer_join =
        thread::spawn(move || draw::Drawer::new(command_receiver, &result_sender).run());
    let mut errors: Vec<String> = Vec::new();

    match drawer_join.join() {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => {
            errors.push(e);
        }
        Err(_) => {
            errors.push(format!("failed to join the consumer thread"));
        }
    }
    match server_join.join() {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => {
            errors.push(e);
        }
        Err(_) => {
            errors.push(String::from("failed to join the server thread"));
        }
    };
    match errors.len() {
        0 => Ok(()),
        _ => Err(errors.join(", ")),
    }
}
