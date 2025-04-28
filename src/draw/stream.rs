use gdk4::Paintable;
use gstreamer::bus;
use gstreamer::prelude::{ElementExt, ElementExtManual, GstBinExtManual};
use gtk4::prelude::*;

pub(crate) struct Stream {
    element: gstreamer::Element,
    bus_watch_guard: bus::BusWatchGuard,
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
        let videoconvert = gstreamer::ElementFactory::make("videoconvert")
            .build()
            .unwrap();
        let aspectratiocrop = gstreamer::ElementFactory::make("aspectratiocrop")
            .property("aspect-ratio", gstreamer::Fraction::new(width, height))
            .build()
            .unwrap();
        let sink = gstreamer::ElementFactory::make("gtk4paintablesink")
            .property("sync", false)
            .build()
            .map_err(|e| e.to_string())?;

        let bin = gstreamer::Bin::new();
        bin.add_many(&[&videoconvert, &aspectratiocrop, &sink])
            .unwrap();
        videoconvert.link(&aspectratiocrop).unwrap();
        aspectratiocrop.link(&sink).unwrap();
        //gstreamer::Element::link_many(&[&videoconvert, &aspectratiocrop, &sink]).unwrap();
        bin.add_pad(
            &gstreamer::GhostPad::with_target(&videoconvert.static_pad("sink").unwrap()).unwrap(),
        )
        .unwrap();

        let paintable = sink.property::<gdk4::Paintable>("paintable");
        let factory = gstreamer::ElementFactory::make("playbin")
            .property("uri", format!("file://{}", file))
            .property("mute", true)
            .property("video-sink", bin)
            .build()
            .map_err(|e| e.to_string())?;

        factory
            .set_state(gstreamer::State::Playing)
            .map_err(|e| format!("failed to set state: {e}"))?;

        let element_ref = factory.downgrade();

        let bus_watch_guard = factory
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
            element: factory,
            bus_watch_guard,
            paintable,
        })
    }
}
