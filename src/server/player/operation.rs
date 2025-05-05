use super::playlist::Playlist;
use super::state;
use super::wallpaper;
use crate::command::{self, make_server_error};
use std::result;
use std::sync::{Mutex, Weak};
///
pub(crate) fn operate(
    state: &Weak<Mutex<state::State>>,
    wallpaper: &impl wallpaper::Wallpaper,
    cmd: &command::Command,
) -> result::Result<serde_json::Value, command::ErrorReason> {
    match cmd {
        command::Command::Stop { monitor } => {
            let monitor = determine_monitor(monitor, wallpaper)?;
            wallpaper.close_window_by_connector(&monitor);
            Ok(serde_json::json!(true))
        }
        command::Command::Shutdown => {
            wallpaper.shutdown();
            Ok(shutdown_result())
        }
        command::Command::Status { .. } => Ok(serde_json::json!(true)),
        // https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
        command::Command::Play { files, monitor } => {
            let connector = match monitor {
                Some(m) => Ok(m.to_string()),
                None => wallpaper
                    .default_connector()
                    .map_err(|e| make_server_error(&e)),
            }?;
            wallpaper.close_window_by_connector(&connector);

            wallpaper.play(state, &connector, &Playlist::new(files, false))?;

            Ok(serde_json::json!(true))
        }
    }
}

///
pub(crate) fn shutdown_result() -> serde_json::Value {
    return serde_json::json!("Server is terminating");
}
///
fn determine_monitor(
    monitor: &Option<String>,
    wallpaper: &impl wallpaper::Wallpaper,
) -> Result<String, command::ErrorReason> {
    match monitor {
        Some(m) => Ok(m.to_string()),
        None => wallpaper
            .default_connector()
            .map_err(|e| make_server_error(&e)),
    }
}
