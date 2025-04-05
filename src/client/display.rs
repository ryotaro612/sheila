use crate::client::client;
use crate::parser;
use serde_json::json;

/**
 *
 */
pub(crate) fn display(
    cli: &impl client::Client,
    id: &str,
    args: parser::DisplayArgs,
) -> Result<String, String> {
    let result = cli
        .send(
            id,
            "display",
            json!({"file": args.file, "monitor": Option::<String>::None}),
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
mod tests {
    use super::display;
    use crate::client::client;
    use crate::parser;
    use mockall::predicate::*;

    #[test]
    fn display_command_sends_a_file_path() {
        let id = "abc";
        let args = parser::DisplayArgs {
            file: "image.png".to_string(),
            monitor: None,
        };
        let mut cli = client::MockClient::new();
        let params = serde_json::json!({
            "file": args.file,
            "monitor": Option::<String>::None,
        });
        cli.expect_send()
            .with(eq(id), eq("display".to_string()), eq(params))
            .returning(|_a, _b, _c| {
                let res: std::result::Result<serde_json::Value, String> = Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": "abc",
                    "result": {}
                }));
                res
            });

        let result = display(&cli, id, args);
        assert_eq!(Ok("".to_string()), result);

        cli.checkpoint();
    }

    #[test]
    fn display_command_returns_error_if_the_server_sent_an_error() {
        let id = "abc";
        let args = parser::DisplayArgs {
            file: "image.png".to_string(),
            monitor: None,
        };
        let mut cli = client::MockClient::new();
        let params = serde_json::json!({
            "file": args.file,
            "monitor": Option::<String>::None,
        });
        cli.expect_send()
            .with(eq(id), eq("display".to_string()), eq(params))
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

        let result = display(&cli, id, args);
        assert!(result.is_err());
        cli.checkpoint();
    }
}
