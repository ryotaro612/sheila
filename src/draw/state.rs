use std::result;
use std::{cell::RefCell, collections::HashMap};

use gdk4::prelude::MonitorExt;
use glib::clone::Upgrade;
use glib::object::ObjectExt;
use gstreamer::{prelude::ElementExt, Element};

use crate::{
    command::{self, make_server_error},
    draw::wallpaper,
};

use super::stream::Stream;
use super::{
    monitor::{connector_name, detect_gdk_monitor},
    stream,
};
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
                if let Some(s) = self.playing.get(&connector) {
                    // TODO handle gracefully
                    s.stop().map_err(|e| make_server_error(&e.to_string()))?;
                    self.playing.remove(&connector);
                    wallpaper.close_window_by_connector(&connector);
                }

                let element = wallpaper.display(&connector, file)?;
                let c = element.downgrade();

                let bus_watch_guard = element
                    .bus()
                    .unwrap()
                    .add_watch_local(move |_bus, msg| {
                        log::debug!("message: {:?}", msg.view());
                        log::debug!("c: {:?}", c.upgrade());
                        match msg.view() {
                            gstreamer::MessageView::Eos(..) => {
                                log::debug!("begin eos:###");
                                if let Some(a) = c.upgrade() {
                                    log::debug!("eos:###");
                                    a.set_state(gstreamer::State::Null).unwrap();
                                    a.set_state(gstreamer::State::Playing).unwrap();
                                }
                            }
                            _ => (),
                        }
                        glib::ControlFlow::Continue
                    })
                    .unwrap();

                self.playing.insert(
                    connector_name,
                    Stream {
                        element,
                        bus_watch_guard,
                    },
                );

                Ok(serde_json::json!({}))
            }
        }
    }
}
