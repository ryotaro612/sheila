use std::collections::HashMap;
use std::result;

use gio::prelude::ApplicationExt;
use serde::de;

use crate::{
    command,
    draw::{monitor::detect_monitors, wallpaper},
};
/**
 *
 */
pub(crate) struct State {
    is_running: bool,
    monitors: HashMap<String, bool>,
}

/**
 *
 */
impl State {
    pub(crate) fn new() -> Self {
        State {
            is_running: true,
            monitors: HashMap::new(),
        }
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
                let monitors = detect_monitors()
                    .map_err(|e| command::ErrorReason::ServerError { reason: e })?;

                let monitor = monitors.first().ok_or(command::ErrorReason::ServerError {
                    reason: "no monitors were detected".to_string(),
                })?;

                log::debug!("monitors: {:?}", monitors);
                Ok(serde_json::json!({}))
            }
        }
    }

    pub(crate) fn up(&mut self, monitor: String) {}
}
