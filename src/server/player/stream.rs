use std::cell::RefCell;
use std::rc;

use gdk4::Paintable;
use glib::WeakRef;
use gstreamer::prelude::ElementExt;
use gstreamer::{bus, Element, Message};
use gtk4::prelude::*;

use super::playlist::Playlist;
///
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

    pub(crate) fn get_playing_file(&self) -> String {
        self.element
            .property::<glib::GString>("current-uri")
            .to_string()
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
        let playlist = rc::Rc::new(RefCell::new(playlist));
        let file = playlist
            .borrow_mut()
            .next()
            .ok_or("No file in the playlist")?;

        let playbin = make_playbin(width, height, &file)?;

        let playbin_ref = playbin.downgrade();
        let bus_watch_guard = playbin
            .bus()
            .unwrap()
            .add_watch_local(move |_bus, msg| {
                if let Err(e) = handle_message(msg, &playbin_ref, &playlist) {
                    log::error!("Failed to handle message: {}", e);
                    on_error();
                }
                glib::ControlFlow::Continue
            })
            .unwrap();

        Ok(Stream {
            element: playbin,
            _bus_watch_guard: bus_watch_guard,
        })
    }
}

fn handle_message(
    msg: &Message,
    playbin_ref: &WeakRef<Element>,
    playlist: &rc::Rc<RefCell<Playlist>>,
) -> Result<(), String> {
    let element = match msg.view() {
        gstreamer::MessageView::Eos(..) => playbin_ref.upgrade(),
        gstreamer::MessageView::Error(e) => return Err(format!("error {}", e)),
        _ => return Ok(()),
    }
    .ok_or("playbin was deleted".to_string())?;
    element
        .set_state(gstreamer::State::Null)
        .map_err(|e| e.to_string())?;
    let file = playlist
        .borrow_mut()
        .next()
        .ok_or("No file in the playlist")?;
    element.set_property("uri", format!("file://{}", file));
    element
        .set_state(gstreamer::State::Playing)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn make_playbin(width: u32, height: u32, file: &str) -> Result<Element, String> {
    let sink = gstreamer::ElementFactory::make("gtk4paintablesink")
        .property("max-lateness", -1 as i64)
        .build()
        .map_err(|e| e.to_string())?;

    let crop = gstreamer::ElementFactory::make("aspectratiocrop")
        .property(
            "aspect-ratio",
            gstreamer::Fraction::new(width as i32, height as i32),
        )
        .build()
        .map_err(|e| e.to_string())?;

    gstreamer::ElementFactory::make("playbin3")
        .property("uri", format!("file://{}", file))
        .property("video-filter", crop)
        .property("video-sink", &sink)
        .build()
        .map_err(|e| e.to_string())
}
