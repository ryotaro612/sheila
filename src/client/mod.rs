use uuid::Uuid;
pub(crate) mod client;
mod display;
pub(crate) mod status;
pub(crate) mod stop;

use crate::parser;

pub(crate) fn run(
    socket: String,
    args: parser::ClientSubCommands,
) -> std::result::Result<(), String> {
    let cli: client::SocketClient = crate::client::client::SocketClient::new(&socket);
    let id = generate_id();
    let res = match args {
        parser::ClientSubCommands::Play(a) => display::display(&cli, &id, a),
        parser::ClientSubCommands::Stop(args) => stop::stop(&cli, id.as_str()),
        parser::ClientSubCommands::Status => status::status(&cli, id.as_str()),
        parser::ClientSubCommands::Shutdown => unimplemented!("implement shutdown"),
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
TODO move
*/
pub(crate) fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod run_tests {
    use super::run;
    use crate::parser;
    use std::io::{Read, Write};
    use std::os::unix::net;
    use std::{env, path, thread};
    use uuid::Uuid;

    #[test]
    fn shutdown() {
        thread::scope(|s| {
            let d = env::temp_dir();
            let socket = path::Path::new(&d)
                .join(Uuid::new_v4().to_string())
                .to_str()
                .unwrap()
                .to_string();

            let socket1 = socket.clone();
            let listener = net::UnixListener::bind(&socket).unwrap();

            let client = s.spawn(move || {
                // let mut stream = net::UnixStream::connect(&socket1).unwrap();
                // stream.write_all("hello".as_bytes()).unwrap();
                // stream.shutdown(std::net::Shutdown::Write).unwrap();

                // let mut buf = String::new();
                // stream.read_to_string(&mut buf).unwrap();
                // stream.shutdown(std::net::Shutdown::Read).unwrap();
                run(socket1, parser::ClientSubCommands::Status).unwrap();
            });

            let server = s.spawn(move || {
                let (mut stream, _) = listener.accept().unwrap(); // ← 接続待ちでブロックする
                let mut buf = String::new();
                stream.read_to_string(&mut buf).unwrap();
                println!("buf: {buf}");
                stream.shutdown(std::net::Shutdown::Read).unwrap();
                stream.write_all("hellow client".as_bytes()).unwrap();
                stream.shutdown(std::net::Shutdown::Write).unwrap();
            });

            client.join().unwrap();
            server.join().unwrap();
        });
    }
}
