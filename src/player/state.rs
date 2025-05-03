use std::collections::HashMap;
use std::result;

use crate::{
    command::{self, make_server_error},
    player::wallpaper,
};

use super::stream::Stream;
/**
 *
 */
pub(crate) struct State {
    is_running: bool,
    playing: HashMap<String, Stream>,
}

/**
 * TODO use signal to detech termination
 */
impl State {
    pub(crate) fn new() -> Self {
        State {
            is_running: true,
            playing: HashMap::new(),
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
            command::Command::Shutdown { .. } => {
                // TODO
                wallpaper.stop();
                self.is_running = false;
                Ok(serde_json::Value::Null)
            }
            command::Command::Status { .. } => Ok(serde_json::json!({})),
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
            command::Command::Play { file, monitor } => {
                if self.is_running == false {
                    return Err(make_server_error("the background service is down"));
                }
                let connector = match monitor {
                    Some(m) => Ok(m.to_string()),
                    None => wallpaper
                        .default_connector()
                        .map_err(|e| make_server_error(&e)),
                }?;
                if let Some(s) = self.playing.get(&connector) {
                    // TODO handle gracefully
                    s.stop().map_err(|e| make_server_error(&e.to_string()))?;
                    self.playing.remove(&connector);
                    wallpaper.close_window_by_connector(&connector);
                }

                let stream = wallpaper.display(&connector, file)?;

                self.playing.insert(connector, stream);

                Ok(serde_json::json!({}))
            }
        }
    }
}
