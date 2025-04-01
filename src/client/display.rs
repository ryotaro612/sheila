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
    cli.send(
        id,
        "display",
        json!({"file": args.file, "monitor": Option::<String>::None}),
    )
    .map_err(|e| e.to_string())?;

    Ok("".to_string())
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
}
