use super::client;
use crate::server::player::operation::shutdown_result;
/// Sends a shutdown request to a server using the client.
pub(crate) fn shutdown(cli: &impl client::Client, id: &str) -> Result<String, String> {
    let res = cli.send_method(id, "shutdown")?;

    if res["result"] == shutdown_result() {
        Ok("".to_string())
    } else {
        Err(format!(
            "error: expected shutdown result, got: {}",
            res["result"]
        ))
    }
}
