use std::collections::HashMap;
use std::result;

use gstreamer::prelude::ElementExt;

use crate::{
    command,
    draw::{monitor::detect_primary_monitor, wallpaper},
};
/**
 *
 */
pub(crate) struct State {
    is_running: bool,
    connector_watch: HashMap<String, gstreamer::bus::BusWatchGuard>,
}

/**
 *
 */
impl State {
    pub(crate) fn new() -> Self {
        State {
            is_running: true,
            connector_watch: HashMap::new(),
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

                let connector = match monitor {
                    Some(m) => m.to_string(),
                    None => {
                        detect_primary_monitor().map_err(|e| command::ErrorReason::ServerError {
                            reason: e.to_string(),
                        })?
                    }
                };
                let element = wallpaper.display(&connector, file)?;
                let watcher = element
                    .bus()
                    .unwrap()
                    .add_watch_local(move |bus, msg| {
                        log::debug!("msg: {:?}", msg);
                        log::debug!("msg: {:?}", msg.view());
                        match msg.view() {
                            gstreamer::MessageView::Eos(..) => {
                                log::debug!("stop");
                                element.set_state(gstreamer::State::Null).unwrap();
                                element.set_state(gstreamer::State::Playing).unwrap();
                            }
                            // MessageView::Error(err) => {
                            //     log::error!("error: {}", err.error());
                            //     factory.set_state(gstreamer::State::Null).unwrap();
                            // }
                            _ => (),
                        }
                        glib::ControlFlow::Continue
                    })
                    .unwrap();

                self.connector_watch.insert(connector, watcher);

                Ok(serde_json::json!({}))
            }
        }
    }
}
