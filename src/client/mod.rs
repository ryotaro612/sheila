use uuid::Uuid;
pub(crate) mod client;
mod play;
mod shutdown;
pub(crate) mod status;
pub(crate) mod stop;

use crate::parser;

pub(crate) fn run(
    socket: &str,
    args: parser::ClientSubCommands,
) -> std::result::Result<(), String> {
    let cli: client::SocketClient = crate::client::client::SocketClient::new(&socket);
    let id = generate_id();
    let res = match args {
        parser::ClientSubCommands::Play(a) => play::play(&cli, &id, a),
        parser::ClientSubCommands::Stop(args) => stop::stop(&cli, id.as_str(), &args),
        parser::ClientSubCommands::Status => status::status(&cli, id.as_str()),
        parser::ClientSubCommands::Shutdown => shutdown::shutdown(&cli, id.as_str()),
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

pub(crate) fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::parser::{PlayArgs, StopArgs};
    use crate::server::request;
    use crate::{command, parser};
    use std::io::{Read, Write};
    use std::os::unix::net;
    use std::panic::resume_unwind;
    use std::{env, fs, panic, path, thread};
    use uuid::Uuid;

    #[test]
    fn shutdown() {
        let res = helper(
            parser::ClientSubCommands::Shutdown,
            command::Command::Shutdown,
            serde_json::json!("Server is terminating"),
        );
        res.unwrap();
    }

    #[test]
    fn status() {
        let res = helper(
            parser::ClientSubCommands::Status,
            command::Command::Status,
            serde_json::json!(true),
        );
        res.unwrap();
    }

    #[test]
    fn stop() {
        let res = helper(
            parser::ClientSubCommands::Stop(StopArgs { monitor: None }),
            command::Command::Stop { monitor: None },
            serde_json::json!(true),
        );
        res.unwrap();
    }

    #[test]
    fn play_monitor_is_optional() {
        let canonical = fs::canonicalize("Cargo.toml")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let res = helper(
            parser::ClientSubCommands::Play(PlayArgs {
                file: "Cargo.toml".to_string(),
                monitor: None,
            }),
            command::Command::Play {
                file: canonical,
                monitor: None,
            },
            serde_json::json!(true),
        );
        res.unwrap();
    }

    #[test]
    fn play_accepts_monitor() {
        let canonical = fs::canonicalize("Cargo.toml")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let res = helper(
            parser::ClientSubCommands::Play(PlayArgs {
                file: "Cargo.toml".to_string(),
                monitor: Some("eDP-1".to_string()),
            }),
            command::Command::Play {
                file: canonical,
                monitor: Some("eDP-1".to_string()),
            },
            serde_json::json!(true),
        );
        res.unwrap();
    }

    #[test]
    fn stop_with_monitor() {
        let res = helper(
            parser::ClientSubCommands::Stop(StopArgs {
                monitor: Some("eDP-1".to_string()),
            }),
            command::Command::Stop {
                monitor: Some("eDP-1".to_string()),
            },
            serde_json::json!(true),
        );
        res.unwrap();
    }

    ///
    fn helper(
        arg_cmd: parser::ClientSubCommands,
        expected: command::Command,
        result: serde_json::Value,
    ) -> Result<(), String> {
        let d = env::temp_dir();
        let socket = path::Path::new(&d)
            .join(Uuid::new_v4().to_string())
            .to_str()
            .unwrap()
            .to_string();
        let sokcet_to_remove = socket.clone();
        let result = panic::catch_unwind(|| {
            thread::scope(|s| {
                let listener = net::UnixListener::bind(&socket).unwrap();
                let client = s.spawn(move || run(&socket, arg_cmd));

                let server = s.spawn(move || {
                    let (mut stream, _) = listener.accept().unwrap();
                    let mut buf = String::new();
                    stream.read_to_string(&mut buf).unwrap();
                    stream.shutdown(std::net::Shutdown::Read).unwrap();
                    let (id, cmd) = request::parse_request(&buf).unwrap();
                    assert_eq!(expected, cmd);
                    let response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result
                    });

                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.shutdown(std::net::Shutdown::Write).unwrap();
                });
                server.join().unwrap();
                client.join().unwrap()
            })
        });
        if let Err(e) = result {
            fs::remove_file(sokcet_to_remove).unwrap();
            resume_unwind(e);
        }
        result.unwrap()
    }
}
