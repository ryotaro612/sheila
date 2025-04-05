use crate::client::client;
use std::result;

/**
 *
 */
pub(crate) fn stop(cli: &impl client::Client, id: &str) -> result::Result<String, String> {
    cli.send_method(id, "stop").map_err(|e| e.to_string())?;
    Ok(String::from(""))
}
