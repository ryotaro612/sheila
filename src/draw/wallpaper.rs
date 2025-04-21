use gstreamer;
use gstreamer::prelude::{ElementExt, ElementExtManual, GstBinExtManual};
use gtk4::{glib, Application, Window};
use gtk4::{prelude::*, Picture};
use gtk4_layer_shell::LayerShell;

use crate::command::{self, make_server_error};

use super::window::{get_rectangle, init_window};

pub(crate) trait Wallpaper {
    fn new_application() -> Result<impl Wallpaper, String>;
    /**
     *
     */
    fn start(&self) -> glib::ExitCode;
    /**
     *
     */
    fn stop(&self);
    /**
     *
     */
    fn display(
        &self,
        monitor: &gdk4::Monitor,
        file: &str,
    ) -> Result<gstreamer::Element, command::ErrorReason>;
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> Result<gtk4::Application, String> {
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();
        app.connect_activate(build_ui);

        gstreamer::init().map_err(|e| e.to_string())?;
        Ok(app)
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }

    fn stop(&self) {
        unsafe {
            gstreamer::deinit();
        }
        self.quit();
    }

    // $  gst-launch-1.0 playbin uri=a.mp4 mute=true video-sink="videoconvert  ! aspectratiocrop aspect-ratio=9/9 ! gtk4paintablesink sync=false"
    // https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
    // TODO close window if failed.
    fn display(
        &self,
        monitor: &gdk4::Monitor,
        file: &str,
    ) -> Result<gstreamer::Element, command::ErrorReason> {
        let window: gtk4::Window =
            init_window(self, monitor).map_err(|e| make_server_error(e.as_str()))?;

        let (width, height) = get_rectangle(&window).map_err(|e| make_server_error(e.as_str()))?;

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
            .map_err(|e| command::ErrorReason::ServerError {
                reason: e.to_string(),
            })?;

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
            .map_err(|e| command::ErrorReason::ServerError {
                reason: e.to_string(),
            })?;

        let picture = Picture::for_paintable(&paintable);

        //window.monitor().map(|m| m.geometry().width());
        window.set_child(Some(&picture));

        factory.set_state(gstreamer::State::Playing).map_err(|e| {
            command::ErrorReason::ServerError {
                reason: format!("failed to set state: {e}"),
            }
        })?;
        window.present();

        Ok(factory)
    }
}
/**
 *
 */
fn build_ui(app: &Application) {
    let _ = Window::builder().application(app).build();
}
