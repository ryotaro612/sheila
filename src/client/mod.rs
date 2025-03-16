use serde;
use serde_json::json;
use std::io;

use crate::parser::{ClientSubCommands, DisplayArgs};

pub(crate) fn run(socket: String, args: crate::parser::ClientArgs) -> io::Result<()> {
    match args.command {
        ClientSubCommands::Display(a) => {
            a.file;
            Ok(())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DisplayRequest {
    method: String,
}

fn make_request(id: String, method: String, params: serde_json::Value) -> serde_json::Value {
    json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": id, })
}

#[test]
fn test_display_request_can_be_represented_as_json_rpc_requet() {
    // arrange
    let args = DisplayArgs {
        file: String::from("image_file"),
    };

    // actual
    let actual = make_request(
        String::from("1"),
        String::from("foobar"),
        json!({"file": "file"}),
    );

    // assert
    let expected = r#"{"jsonrpc":"2.0","method":"display","params":{"file":"file"}}"#;
    assert_eq!(serde_json::to_string(&actual).unwrap(), expected);
}
