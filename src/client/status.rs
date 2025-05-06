/**
 *
 */
use crate::client::client;
use std::result;
/**
 * sneds a status command by the client.
 */
pub(crate) fn status(cli: &impl client::Client, id: &str) -> result::Result<String, String> {
    let res = cli.send_method(id, "status").map_err(|e| e.to_string())?;
    Ok(res["result"].to_string())
}
