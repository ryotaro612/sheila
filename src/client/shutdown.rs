use super::client;

pub(crate) fn shutdown(cli: &impl client::Client, id: &str) -> Result<String, String> {
    let res = cli.send_method(id, "shutdown")?;
    match res["result"] {
        serde_json::Value::Bool(true) => Ok("".to_string()),
        _ => Err(res.to_string()),
    }
}
