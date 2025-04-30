use std::result;
use std::thread::sleep;
use std::time::Duration;
mod handler;
pub(crate) mod request;
mod response;
mod server;
use crate::client as c;
use crate::client::client;
use crate::client::status;
use crate::client::stop;
use crate::command;
use crate::draw;
use std::sync::mpsc;
use std::thread;

///  Initializes the log system.
pub(crate) fn run(socket: String) -> result::Result<(), String> {
    thread::scope(move |s| {
        let (command_sender, command_receiver) = mpsc::channel::<command::Command>();
        let (result_sender, result_receiver) =
            mpsc::channel::<Result<serde_json::Value, command::ErrorReason>>();

        let server_socket = socket.clone();
        let server_join = s.spawn(move || {
            let server = server::Server::new(
                &server_socket,
                handler::DefaultHandler::new(&command_sender, &result_receiver),
            );
            server.start().map_err(|e| e.to_string())
        });
        let drawer_join =
            s.spawn(move || draw::Drawer::new(command_receiver, &result_sender).run());

        let healcheck_socket = socket.clone();
        let healthcheck = s.spawn(move || {
            let client = client::SocketClient::new(&healcheck_socket);
            loop {
                sleep(Duration::from_secs(5));
                if let Err(e) = status::status(&client, &c::generate_id()) {
                    log::debug!("status error: {e}");
                    // TODO shutdown
                    //stop::stop(&client, &c::generate_id()).unwrap();
                    break;
                }
            }
        });

        healthcheck.join().unwrap();

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
    })
}
