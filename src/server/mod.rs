use std::io;
use std::io::Error;
use std::os::unix::net::UnixListener;
pub mod server;

/**
 *  Initialize the log system
*/
pub(crate) fn run(socket: String, args: crate::parser::ServerArgs) -> io::Result<()> {
    let listner = UnixListener::bind(socket)?;

    let mut result: Result<(), Error> = Ok(());
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {}
            Err(err) => {
                result = Err(err);
                break;
            }
        };
    }
    result
}
