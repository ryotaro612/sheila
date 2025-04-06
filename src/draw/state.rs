use std::collections::HashMap;
use std::result;

use crate::{
    command,
    draw::{monitor::detect_primary_monitor, wallpaper},
};
/**
 *
 */
pub(crate) struct State {
    is_running: bool,
}

/**
 *
 */
impl State {
    pub(crate) fn new() -> Self {
        State { is_running: true }
    }

    /**
     * Stop command returns OK.
     */
    pub(crate) fn execute(
        &mut self,
        wallpaper: &impl wallpaper::Wallpaper,
        cmd: &command::Command,
    ) -> result::Result<serde_json::Value, command::ErrorReason> {
        match cmd {
            command::Command::Stop { .. } => {
                wallpaper.stop();
                self.is_running = false;
                Ok(serde_json::Value::Null)
            }
            command::Command::Status { .. } => Ok(serde_json::json!({})),
            command::Command::Display { file, monitor } => {
                if self.is_running == false {
                    return Err(command::ErrorReason::ServerError {
                        reason: "the background service is down".to_string(),
                    });
                }
                log::debug!("display command: file: {}, monitor: {:?}", file, monitor);

                let connector = match monitor {
                    Some(m) => m.to_string(),
                    None => {
                        detect_primary_monitor().map_err(|e| command::ErrorReason::ServerError {
                            reason: e.to_string(),
                        })?
                    }
                };
                wallpaper.display(&connector, file)?;

                Ok(serde_json::json!({}))
            }
        }
    }
}
