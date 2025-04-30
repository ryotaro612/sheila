use crate::client::client;
use crate::parser;
use std::result;

///
pub(crate) fn stop(
    cli: &impl client::Client,
    id: &str,
    parser::StopArgs { monitor }: &parser::StopArgs,
) -> result::Result<String, String> {
    let response = cli.send(
        id,
        "stop",
        serde_json::json!({
            "monitor": monitor
        }),
    )?;
    match response["result"] {
        serde_json::Value::Bool(true) => Ok(String::from("")),
        _ => Err(format!("{}", response["error"])),
    }
}
