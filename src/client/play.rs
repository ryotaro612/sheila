use std::path::Path;

use crate::client::client;
use crate::parser;
use serde_json::json;

/// Returns Ok("") if the client have received a success result.
pub(crate) fn play(
    cli: &impl client::Client,
    id: &str,
    args: parser::PlayArgs,
) -> Result<String, String> {
    let mut files: Vec<String> = vec![];

    for file in args.files {
        let path_buf = Path::new(&file).canonicalize().map_err(|e| e.to_string())?;
        let path_str = path_buf.to_str().ok_or(format!(
            "failed to make path_buf to str. path_buf: {:?}",
            path_buf
        ))?;
        files.push(path_str.to_string());
    }

    let result = cli
        .send(
            id,
            "play",
            json!({
                "files": files,
                "monitor": args.monitor
            }),
        )
        .map_err(|e| e.to_string())?;

    match result.get("error") {
        Some(e) => {
            let message = e.get("message").ok_or("message was not found")?;
            let message_str = message.as_str().ok_or("message is not a string")?;
            Err(message_str.to_string())
        }
        None => Ok("".to_string()),
    }
}

#[cfg(test)]
mod display_tests {
    use super::play;
    use crate::client::client;
    use crate::parser;
    use mockall::predicate::*;

    #[test]
    fn returns_error_on_server_error() {
        let id = "abc";
        let args = parser::PlayArgs {
            files: vec!["/movie.mp4".to_string()],
            monitor: None,
        };
        let mut cli = client::MockClient::new();
        let params = serde_json::json!({
            "files": args.files,
            "monitor":serde_json::Value::Null
        });

        cli.expect_send()
            .with(eq(id), eq("play".to_string()), eq(params))
            .returning(|_a, _b, _c| {
                Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id.to_string(),
                    "error": {
                        "code": -32000,
                        "message": "error"
                    }
                }))
            });

        let result = play(&cli, id, args);
        assert!(result.is_err());
        cli.checkpoint();
    }
}
