use std::result;

use gio::prelude::ApplicationExt;

use crate::{command, draw::wallpaper};

pub(crate) struct State {}

impl State {
    pub(crate) fn new() -> Self {
        State {}
    }

    /**
     * Stop command returns OK.
     */
    pub(crate) fn execute(
        &self,
        wallpaper: &impl wallpaper::Wallpaper,
        cmd: &command::Command,
    ) -> result::Result<serde_json::Value, command::ErrorReason> {
        match cmd {
            command::Command::Stop { .. } => {
                wallpaper.application().quit();
                Ok(serde_json::Value::Null)
            }
            command::Command::Status { .. } => Ok(serde_json::json!({})),
        }
    }
}
