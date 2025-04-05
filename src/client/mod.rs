use uuid::Uuid;
pub(crate) mod client;
mod display;
pub(crate) mod status;
pub(crate) mod stop;

use crate::parser::ClientSubCommands;

pub(crate) fn run(
    socket: String,
    args: crate::parser::ClientSubCommands,
) -> std::result::Result<(), String> {
    let cli = crate::client::client::SocketClient::new(&socket);
    let id = generate_id();
    let res = match args {
        ClientSubCommands::Display(a) => display::display(&cli, &id, a),
        ClientSubCommands::Stop => stop::stop(&cli, id.as_str()),
        ClientSubCommands::Status => status::status(&cli, id.as_str()),
    };
    match res {
        Ok(s) => {
            if !s.is_empty() {
                println!("{}", s);
            }
            Ok(())
        }
        Err(s) => Err(s),
    }
}
/**
 *
 */
pub(crate) fn generate_id() -> String {
    Uuid::new_v4().to_string()
}
