use std::collections::HashMap;
use std::result;

use gdk4::prelude::MonitorExt;
use glib::object::ObjectExt;
use gstreamer::{prelude::ElementExt, Element};

use crate::{
    command::{self, make_server_error},
    draw::wallpaper,
};

use super::{
    monitor::{connector_name, detect_gdk_monitor},
    stream,
};
/**
 *
 */
pub(crate) struct State {
    is_running: bool,
    connector_watch: HashMap<String, Element>,
}

/**
 * TODO use signal to detech termination
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
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
            command::Command::Display { file, monitor } => {
                if self.is_running == false {
                    return Err(make_server_error("the background service is down"));
                }
                let connector = match monitor {
                    Some(m) => Ok(m.to_string()),
                    None => wallpaper
                        .default_connector()
                        .map_err(|e| make_server_error(&e)),
                }?;
                if let Some(element) = self.connector_watch.get(&connector) {
                    stream::stop(element);
                    self.connector_watch.remove(&connector);
                }
                wallpaper.close_window_by_connector(&connector);

                let gdk_monitor =
                    detect_gdk_monitor(monitor).map_err(|e| make_server_error(e.as_str()))?;
                let connector_name =
                    connector_name(&gdk_monitor).map_err(|e| make_server_error(e.as_str()))?;

                let element = wallpaper.display(&gdk_monitor, file)?;
                let element_weak = element.downgrade();
                let _ = element
                    .bus()
                    .unwrap()
                    .add_watch_local(move |_bus, msg| {
                        // log::debug!("msg: {:?}", msg);
                        // log::debug!("msg: {:?}", msg.view());
                        match msg.view() {
                            gstreamer::MessageView::Eos(..) => {
                                log::debug!("stop");
                                if let Some(element) = element_weak.upgrade() {
                                    element.set_state(gstreamer::State::Null).unwrap();
                                    element.set_state(gstreamer::State::Playing).unwrap();
                                }
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

                self.connector_watch.insert(connector_name, element);

                Ok(serde_json::json!({}))
            }
        }
    }
}
