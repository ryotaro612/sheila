use gdk4::Paintable;
use gstreamer::bus;
use gstreamer::prelude::{ElementExt, ElementExtManual, GstBinExtManual};
use gtk4::prelude::*;
/**
* gst-launch-1.0 -v   filesrc location=~/a.mp4 !   qtdemux name=demux   demux.video_0 ! queue ! vaapidecodebin ! videoconvert !  aspectratiocrop  aspect-ratio=16/9 ! gtk4paintablesink

*/
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

/*

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Picture};
use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, Element};
use gstreamer::MessageView;
use gstreamer_gtk4::Gtk4PaintableSink;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GTK and GStreamer
    gtk4::init()?;
    gstreamer::init()?;

    let app = Application::builder()
        .application_id("com.example.gstreamer_loop")
        .build();

    app.connect_activate(build_ui);

    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    // ファイル名を指定
    let filename = "/path/to/your/video.mp4";

    // ウィンドウ作成
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Seamless Loop Video")
        .default_width(800)
        .default_height(600)
        .build();

    // Pictureウィジェット作成
    let picture = Picture::new();
    window.set_child(Some(&picture));

    // playbin作成
    let playbin = ElementFactory::make("playbin", Some("playbin"))
        .expect("Could not create playbin");

    // ファイル指定
    let uri = format!("file://{}", filename);
    playbin.set_property("uri", &uri).unwrap();

    // gtk4paintablesinkを作って映像出力に設定
    let sink = ElementFactory::make("gtk4paintablesink", Some("videosink"))
        .expect("Could not create gtk4paintablesink");
    sink.set_property("force-aspect-ratio", &true).unwrap();

    playbin.set_property("video-sink", &sink).unwrap();

    // gtk4paintablesinkからpaintableを取ってPictureにセット
    let sink = sink.dynamic_cast::<Gtk4PaintableSink>().expect("Not a Gtk4PaintableSink");
    if let Some(paintable) = sink.paintable() {
        picture.set_paintable(Some(&paintable));
    }

    // メッセージバスでシグナルを受け取る
    let bus = playbin.bus().unwrap();
    let playbin_clone = playbin.clone();
    bus.add_watch_local(move |_, msg| {
        match msg.view() {
            MessageView::Eos(..) => {
                // End Of Stream: もう一度最初から再生！
                println!("EOS received, seeking to start");
                let _ = playbin_clone.seek_simple(
                    gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT,
                    0, // position 0 (start)
                );
            }
            MessageView::Error(err) => {
                println!("Error: {}", err.error());
            }
            _ => (),
        }
        glib::Continue(true)
    }).expect("Failed to add bus watch");

    window.show();

    // 再生開始
    playbin.set_state(gstreamer::State::Playing).unwrap();
}
 */
