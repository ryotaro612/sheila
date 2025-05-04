use gdk4::Paintable;
use gstreamer::bus;
use gstreamer::prelude::{ElementExt, GstBinExtManual};
use gtk4::prelude::*;
use std::rc;
/**
* gst-launch-1.0 -v   filesrc location=~/a.mp4 !   qtdemux name=demux   demux.video_0 ! queue ! vaapidecodebin ! videoconvert !  aspectratiocrop  aspect-ratio=16/9 ! gtk4paintablesink
 gst-launch-1.0 -v \
  playbin \
  uri=file:///home/youruser/file.mp4 \
  video-filter="aspectratiocrop aspect-ratio=16/9" \
  video-sink=gtk4paintablesink
*/
#[derive(Debug, Clone)]
pub(crate) struct Stream {
    element: gstreamer::Element,
    bus_watch_guard: rc::Rc<bus::BusWatchGuard>,
    paintable: Paintable,
}

impl Stream {
    pub(crate) fn stop(
        &self,
    ) -> Result<gstreamer::StateChangeSuccess, gstreamer::StateChangeError> {
        self.element.set_state(gstreamer::State::Null)
    }

    pub(crate) fn paintable(&self) -> Paintable {
        self.paintable.clone()
    }

    pub(crate) fn new(file: &str, width: i32, height: i32) -> Result<Stream, String> {
        let neg: i64 = -1;
        let sink = gstreamer::ElementFactory::make("gtk4paintablesink")
            .property("max-lateness", neg)
            .build()
            .map_err(|e| e.to_string())?;

        let crop = gstreamer::ElementFactory::make("aspectratiocrop")
            .property("aspect-ratio", gstreamer::Fraction::new(width, height))
            .build()
            .map_err(|e| e.to_string())?;

        let playbin = gstreamer::ElementFactory::make("playbin")
            .property("uri", format!("file://{}", file))
            .property("video-filter", crop)
            .property("video-sink", &sink)
            .build()
            .map_err(|e| e.to_string())?;

        let paintable = sink.property::<gdk4::Paintable>("paintable");

        playbin
            .set_state(gstreamer::State::Playing)
            .map_err(|e| format!("failed to set state: {e}"))?;

        let element_ref = playbin.downgrade();

        let bus_watch_guard = playbin
            .bus()
            .unwrap()
            .add_watch_local(move |_bus, msg| {
                log::debug!("message: {:?}", msg.view());
                log::debug!("c: {:?}", element_ref.upgrade());
                match msg.view() {
                    gstreamer::MessageView::Eos(..) => {
                        log::debug!("begin eos:###");
                        if let Some(a) = element_ref.upgrade() {
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

        Ok(Stream {
            element: playbin,
            bus_watch_guard: rc::Rc::new(bus_watch_guard),
            paintable,
        })
    }
}
