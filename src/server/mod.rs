use std::{io, result};
use std::os::unix::net::UnixListener;
pub mod server;

// sync=false is required playbin

/**
 *  Initialize the log system
*/
pub(crate) fn run(socket: String, args: crate::parser::ServerArgs) -> result::Result<(), String> {
    let listner = UnixListener::bind(socket).map_err(|e| e.to_string())?;

    let mut result: Result<(), String> = Ok(());
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {}
            Err(err) => {
                result = Err(err.to_string());
                break;
            }
        };
    }
    result
}
