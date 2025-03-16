use std::io::Read;
use serde_json::json;
use std::io::Write;
use std::net;
use std::result;
use std::io;

pub(crate) trait Client {
     fn send(&self, id:String, method: &str, params: serde_json::Value) -> result::Result<(), io::Error>;
     fn send_method(&self, id:String, method: &str) -> result::Result<(), io::Error>;
}

pub(crate) struct SocketClient {
    socket: String,
}

impl Client for SocketClient {
    fn send(&self, id: String, method: &str, params: serde_json::Value) -> result::Result<(), io::Error> {
        let mut stream = std::os::unix::net::UnixStream::connect(&self.socket)?;

        let request  =json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id, });

        stream.write_all(request.to_string().as_bytes())?;
        stream.shutdown(net::Shutdown::Write)?;
        // stream.read_timeout();
        let mut message = String::new();
        stream.read_to_string(&mut message);
        println!("{message}");
        Ok(())
    }

     fn send_method(&self, id:String, method: &str) -> result::Result<(), io::Error> {
        let mut stream = std::os::unix::net::UnixStream::connect(&self.socket)?;

        let request  =json!({
            "jsonrpc": "2.0",
            "method": method,
            "id": id, });

        stream.write_all(request.to_string().as_bytes())?;
        stream.shutdown(net::Shutdown::Write)?;
        // stream.read_timeout();
        let mut message = String::new();
        stream.read_to_string(&mut message);
        println!("{message}");
        Ok(())
     }
}

impl SocketClient {

        pub(crate) fn new(socket: &String) -> Self {
        SocketClient {
            socket: socket.clone(),
        }
    }
}



    