use std::result;

use gio::prelude::ApplicationExt;

use crate::{
    command,
    draw::{monitor::detect_monitors, wallpaper},
};

pub(crate) struct State {
    is_running: bool,
}

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
                        reason: "the wallpaper is closed.".to_string(),
                    });
                }
                log::debug!("display command: file: {}, monitor: {:?}", file, monitor);
                let monitors = detect_monitors();
                log::debug!("monitors: {:?}", monitors);
                Ok(serde_json::json!({}))
            }
        }
    }
}
