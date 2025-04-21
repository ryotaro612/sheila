use std::path::Path;

use crate::client::client;
use crate::parser;
use serde_json::json;

/**
 * Returns Ok("") if the client have received a success result.
 */
pub(crate) fn display(
    cli: &impl client::Client,
    id: &str,
    args: parser::DisplayArgs,
) -> Result<String, String> {
    let p = Path::new(&args.file)
        .canonicalize()
        .map_err(|e| e.to_string())?
        .to_str()
        .ok_or("failed to make the file path canonical")?
        .to_string();

    let result = cli
        .send(
            id,
            "display",
            json!(
                {"file": p,
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
    use super::display;
    use crate::client::client;
    use crate::parser;
    use mockall::predicate::*;
    use serde_json::json;

    #[test]
    fn sends_file_path() {
        // arrange
        let id = "abc";
        let args = parser::DisplayArgs {
            file: "Cargo.toml".to_string(),
            monitor: None,
        };
        let mut cli = client::MockClient::new();

        cli.expect_send().returning(|id, method, params| {
            assert_eq!("abc", id);
            assert_eq!("display", method);
            let f = params["file"].as_str().unwrap();
            assert!(f.ends_with("Cargo.toml"));
            assert_eq!(json!(null), params["monitor"]);

            let res: std::result::Result<serde_json::Value, String> = Ok(serde_json::json!({
                "jsonrpc": "2.0",
                "id": "abc",
                "result": {}
            }));
            res
        });
        // act
        let result = display(&cli, id, args);

        // assert
        assert_eq!(Ok("".to_string()), result);
        cli.checkpoint();
    }

    #[test]
    fn sends_monitor() {
        // arrange
        let id = "abc";
        let args = parser::DisplayArgs {
            file: "Cargo.toml".to_string(),
            monitor: Some("eDP-1".to_string()),
        };
        let mut cli = client::MockClient::new();

        cli.expect_send().returning(|id, method, params| {
            assert_eq!("abc", id);
            assert_eq!("display", method);
            let f = params["file"].as_str().unwrap();
            assert!(f.ends_with("Cargo.toml"));
            assert_eq!(json!("eDP-1"), params["monitor"]);

            let res: std::result::Result<serde_json::Value, String> = Ok(serde_json::json!({
                "jsonrpc": "2.0",
                "id": "abc",
                "result": {}
            }));
            res
        });
        // act
        let result = display(&cli, id, args);

        // assert
        assert_eq!(Ok("".to_string()), result);
        cli.checkpoint();
    }

    #[test]
    fn returns_error_on_server_error() {
        let id = "abc";
        let args = parser::DisplayArgs {
            file: "/image.png".to_string(),
            monitor: None,
        };
        let mut cli = client::MockClient::new();
        let params = serde_json::json!({
            "file": args.file,
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
