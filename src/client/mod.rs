use uuid::Uuid;
use serde_json::json;
use std::result;
mod client;
use crate::client::client as sheila_client;


use crate::parser::{ ClientSubCommands, DisplayArgs};


pub(crate) fn run(socket: String, args: crate::parser::ClientSubCommands) -> std::result::Result<(), String> {
    let cli = crate::client::client::SocketClient::new(&socket);
    let id = generate_id();
    match args {
       ClientSubCommands::Display(a) => {
        display(&cli, id, a)
       } 
       ClientSubCommands::Stop => {
        stop(&cli, id)
       }
    }

}

fn generate_id() -> String { 
Uuid::new_v4().to_string()
}

fn display(cli: &impl sheila_client::Client, id: String,       a: DisplayArgs) -> result::Result<(), String> {
    let response = cli.send(id, "display", json!({"file": a.file})).map_err(|e| e.to_string())?;
    Ok(())
}
fn stop(cli: &impl sheila_client::Client, id: String) -> result::Result<(), String> {
    let response = cli.send_method(id, "stop").map_err(|e| e.to_string())?;
    Ok(())
}


// fn receive(socket: String) -> result::Result<serde_json::Value, io::Error> {
//     let mut stream = std::os::unix::net::UnixStream::connect(socket)?;
//     let mut response = String::new();

//       stream.read_to_string(&mut message);
//     stream.read_to_string(&mut response)?;
//     Ok(serde_json::from_str(&response).unwrap())

// }


// fn make_request(id: String, args: crate::parser::ClientSubCommands) -> result::Result<serde_json::Value, String> {
//     match args {
//         ClientSubCommands::Display(a) => {
//             let value = json!({"file": a.file});
//             let res: Result<(&str, serde_json::Value), String>  = ;
// Ok(json!({
//         "jsonrpc": "2.0",
//         "method": method,
//         "params": method_params.1,
//         "id": id, }))
//         }
//         ClientSubCommands::Stop => {

//         }
//     }?;

    
// }

// #[test]
// fn test_display_request_can_be_represented_as_json_rpc_requet() {
//     // arrange
//     let args = DisplayArgs {
//         file: String::from("image_file"),
//     };

//     // actual
//     let actual =  make_request(
//         String::from("1"),
//         ClientSubCommands::Display(args),
//     );

//     // assert
//     let expected = json!({ "jsonrpc":"2.0", "id": "1",  "method":"display","params":{"file":"image_file"}});

//     assert_eq!(expected, actual.unwrap());
// }
