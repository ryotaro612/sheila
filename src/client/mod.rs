use uuid::Uuid;
use serde_json::json;
use std::io;
use std::io::Read;
use std::net;
use std::io::Write;
use std::result;

use crate::parser::{ClientSubCommands, DisplayArgs};


pub(crate) fn run(socket: String, args: crate::parser::ClientSubCommands) -> std::result::Result<(), String> {
    let id = Uuid::new_v4().to_string();
    let request = make_request(id, args)?;
    send(socket, request).map_err(|e| e.to_string())
}

fn send(socket: String, request: serde_json::Value) -> result::Result<(), io::Error> {
    let mut stream = std::os::unix::net::UnixStream::connect(socket)?;
    stream.write_all(request.to_string().as_bytes())?;
    stream.shutdown(net::Shutdown::Write)?;
    // stream.read_timeout();
    let mut message = String::new();
    stream.read_to_string(&mut message);
    println!("{message}");
    Ok(())
}

// fn receive(socket: String) -> result::Result<serde_json::Value, io::Error> {
//     let mut stream = std::os::unix::net::UnixStream::connect(socket)?;
//     let mut response = String::new();

//       stream.read_to_string(&mut message);
//     stream.read_to_string(&mut response)?;
//     Ok(serde_json::from_str(&response).unwrap())

// }


fn make_request(id: String, args: crate::parser::ClientSubCommands) -> result::Result<serde_json::Value, String> {
    let method_params        =   match args {
        ClientSubCommands::Display(a) => {
            let value = json!({"file": a.file});
            let res: Result<(&str, serde_json::Value), String>  = Ok(("display", value));
            res
        }
    }?;

    Ok(json!({
        "jsonrpc": "2.0",
        "method": method_params.0,
        "params": method_params.1,
        "id": id, }))
}

#[test]
fn test_display_request_can_be_represented_as_json_rpc_requet() {
    // arrange
    let args = DisplayArgs {
        file: String::from("image_file"),
    };

    // actual
    let actual =  make_request(
        String::from("1"),
        ClientSubCommands::Display(args),
    );

    // assert
    let expected = json!({ "jsonrpc":"2.0", "id": "1",  "method":"display","params":{"file":"image_file"}});

    assert_eq!(expected, actual.unwrap());
}
