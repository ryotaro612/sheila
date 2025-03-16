use serde;
use uuid::Uuid;
use serde_json::json;
use std::io;

use crate::parser::{ClientSubCommands, DisplayArgs};

struct MethodParms {
    method: String,
    params: serde_json::Value,
}

pub(crate) fn run(socket: String, args: crate::parser::ClientArgs) -> std::result::Result<(), String> {
    let method_params        =   match args.command {
        ClientSubCommands::Display(a) => {
            let value = json!({"file": a.file});
            let m = MethodParms{method: String::from("display"), params: value};
            Ok(("display", value))
        }
        _ => {
            Err(String::from("Not implemented"))
        }
    }?;

    let id = Uuid::new_v4().to_string();
    let request = make_request(id, method_params.0, method_params.1);
    Ok(())
}


fn make_request(id: String, method: &str, params: serde_json::Value) -> serde_json::Value {
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
