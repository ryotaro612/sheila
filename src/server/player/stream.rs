use std::cell::RefCell;
use std::rc;

use gdk4::Paintable;
use gstreamer::prelude::ElementExt;
use gstreamer::{bus, Element};
use gtk4::prelude::*;

use super::playlist::Playlist;
/**

*/
#[derive(Debug)]
pub(crate) struct Stream {
    element: gstreamer::Element,
    _bus_watch_guard: bus::BusWatchGuard,
}

impl Stream {
    pub(crate) fn stop(
        &self,
    ) -> Result<gstreamer::StateChangeSuccess, gstreamer::StateChangeError> {
        self.element.set_state(gstreamer::State::Null)
    }

    pub(crate) fn paintable(&self) -> Paintable {
        self.element
            .property::<Element>("video-sink")
            .property::<gdk4::Paintable>("paintable")
    }

    pub(crate) fn play(
        &self,
    ) -> Result<gstreamer::StateChangeSuccess, gstreamer::StateChangeError> {
        self.element.set_state(gstreamer::State::Playing)
    }
    ///  gst-launch-1.0 -v playbin uri=file:///home/youruser/file.mp4 video-filter="aspectratiocrop aspect-ratio=16/9" video-sink=gtk4paintablesink
    pub(crate) fn new(
        playlist: Playlist,
        width: u32,
        height: u32,
        on_error: impl Fn() -> () + 'static,
    ) -> Result<Stream, String> {
        let neg: i64 = -1;
        let sink = gstreamer::ElementFactory::make("gtk4paintablesink")
            .property("max-lateness", neg)
            .build()
            .map_err(|e| e.to_string())?;

        let crop = gstreamer::ElementFactory::make("aspectratiocrop")
            .property(
                "aspect-ratio",
                gstreamer::Fraction::new(width as i32, height as i32),
            )
            .build()
            .map_err(|e| e.to_string())?;

        let playlist = rc::Rc::new(RefCell::new(playlist));
        let file = playlist
            .borrow_mut()
            .next()
            .ok_or("No file in the playlist")?;

        let playbin = gstreamer::ElementFactory::make("playbin")
            .property("uri", format!("file://{}", file))
            .property("video-filter", crop)
            .property("video-sink", &sink)
            .build()
            .map_err(|e| e.to_string())?;

        let playbin_ref = playbin.downgrade();
        let bus_watch_guard = playbin
            .bus()
            .unwrap()
            .add_watch_local(move |_bus, msg| {
                match msg.view() {
                    gstreamer::MessageView::Eos(..) => {
                        if let Some(elm) = playbin_ref.upgrade() {
                            match elm.set_state(gstreamer::State::Null) {
                                Ok(_) => {
                                    if let Some(file) = playlist.borrow_mut().next() {
                                        elm.set_property("uri", format!("file://{}", file));
                                        if let Err(e) = elm.set_state(gstreamer::State::Playing) {
                                            log::error!("Failed to set state to Playing: {}", e);
                                            on_error();
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to set state to Null: {}", e);
                                    on_error();
                                }
                            }
                        } else {
                            log::error!("the element was deleted.");
                            on_error()
                        }
                    }
                    gstreamer::MessageView::Error(..) => on_error(),
                    _ => (),
                };
                glib::ControlFlow::Continue
            })
            .unwrap();

        Ok(Stream {
            element: playbin,
            _bus_watch_guard: bus_watch_guard,
        })
    }
}
