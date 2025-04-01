use crate::client::client as sheila_client;
use serde_json::json;
use std::result;
use uuid::Uuid;
mod client;
mod display;

use crate::parser::{ClientSubCommands, DisplayArgs};

pub(crate) fn run(
    socket: String,
    args: crate::parser::ClientSubCommands,
) -> std::result::Result<(), String> {
    let cli = crate::client::client::SocketClient::new(&socket);
    let id = generate_id();
    match args {
        ClientSubCommands::Display(a) => display(&cli, &id, a),
        ClientSubCommands::Stop => stop(&cli, id),
    }
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

fn display(
    cli: &impl sheila_client::Client,
    id: &str,
    a: DisplayArgs,
) -> result::Result<(), String> {
    let _response = cli
        .send(id, "display", json!({"file": a.file}))
        .map_err(|e| e.to_string())?;
    Ok(())
}
fn stop(cli: &impl sheila_client::Client, id: String) -> result::Result<(), String> {
    let _response = cli.send_method(id, "stop").map_err(|e| e.to_string())?;
    Ok(())
}
