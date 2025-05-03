use std::result;
mod handler;
pub(crate) mod request;
mod response;
mod server;
use crate::command;
use crate::player;
use std::sync::mpsc;
use std::thread;

///  Initializes the log system.
pub(crate) fn run(socket: &str) -> result::Result<(), String> {
    thread::scope(move |s| {
        let (command_sender, command_receiver) = mpsc::channel::<command::Command>();
        let (result_sender, result_receiver) =
            mpsc::channel::<Result<serde_json::Value, command::ErrorReason>>();

        let server_join = s.spawn(move || {
            let server = server::Server::new(
                &socket,
                handler::DefaultHandler::new(&command_sender, &result_receiver),
            );
            server.start().map_err(|e| e.to_string())
        });

        let drawer_join =
            s.spawn(move || player::Drawer::new(command_receiver, &result_sender).run());

        let mut errors: Vec<String> = Vec::new();

        let handles = vec![("server", server_join), ("player", drawer_join)];

        for (name, handle) in handles {
            match handle.join() {
                Ok(Ok(_)) => (),
                Ok(Err(e)) => {
                    errors.push(e);
                }
                Err(_) => {
                    errors.push(format!("failed to join the {name} thread"));
                }
            }
        }
        match errors.len() {
            0 => Ok(()),
            _ => Err(errors.join(", ")),
        }
    })
}
