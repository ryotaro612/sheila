use std::thread;

use gstreamer;
use gstreamer::prelude::{ElementExt, ElementExtManual, GstBinExtManual};
use gtk4::{glib, Application, Window};
use gtk4::{prelude::*, Picture};
use gtk4_layer_shell::LayerShell;

use crate::command;

use super::monitor::detect_gdk_monitor;

pub(crate) trait Wallpaper {
    fn new_application() -> Result<impl Wallpaper, String>;
    fn stop(&self);
    fn start(&self) -> glib::ExitCode;
    fn display(&self, monitor: &str, file: &str) -> Result<(), command::ErrorReason>;
    fn find_window(&self, monitor: &str) -> Option<Window>;
    fn init_window(&self, monitor: &str) -> Result<Window, String>;
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> Result<gtk4::Application, String> {
        //app.connect_activate(build_ui);
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();
        app.connect_activate(build_ui);

        gstreamer::init().map_err(|e| e.to_string())?;
        Ok(app)
    }

    fn stop(&self) {
        self.quit();
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }

    /**
    * $  gst-launch-1.0 playbin uri=a.mp4 mute=true video-sink="videoconvert  ! aspectratiocrop aspect-ratio=9/9 ! gtk4paintablesink sync=false"

    * https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
    */
    fn display(&self, connector: &str, file: &str) -> Result<(), command::ErrorReason> {
        let window: gtk4::Window = match self.find_window(connector) {
            Some(window) => Ok::<Window, command::ErrorReason>(window),
            None => match self.init_window(connector) {
                Ok(window) => Ok(window),
                Err(e) => Err(command::ErrorReason::ServerError {
                    reason: format!("failed to create a window. {e}"),
                }),
            },
        }?;
        let videoconvert = gstreamer::ElementFactory::make("videoconvert")
            .build()
            .unwrap();
        let aspectratiocrop = gstreamer::ElementFactory::make("aspectratiocrop")
            .property("aspect-ratio", gstreamer::Fraction::new(16, 10))
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
        window.set_child(Some(&picture));

        //let bus = factory.bus().ok_or("failed to get a bus")?;

        factory.set_state(gstreamer::State::Playing).map_err(|e| {
            command::ErrorReason::ServerError {
                reason: format!("failed to set state: {e}"),
            }
        })?;

        thread::spawn(|| {
            factory
                .bus()
                .unwrap()
                .add_watch_local(move |bus, msg| {
                    log::debug!("msg: {:?}", msg);
                    match msg.view() {
                        gstreamer::MessageView::Eos(..) => {
                            log::debug!("stop");
                            factory.set_state(gstreamer::State::Null).unwrap();
                            factory.set_state(gstreamer::State::Playing).unwrap();
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
        });

        window.present();

        Ok(())
    }

    fn find_window(&self, connector: &str) -> Option<Window> {
        let windows: Vec<Window> = self.windows();
        let found: Vec<Window> = windows
            .iter()
            .filter(|w| w.is_visible())
            .filter_map(|window| {
                if window.monitor()?.connector()?.to_string() == connector {
                    Some(window.clone())
                } else {
                    None
                }
            })
            .collect();
        found.get(0).map(|w| w.clone())
    }

    fn init_window(&self, connector: &str) -> Result<Window, String> {
        let window = Window::builder().application(self).build();
        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Bottom);

        let monitor = detect_gdk_monitor(connector)?;
        window.set_monitor(Some(&monitor));
        window.set_anchor(gtk4_layer_shell::Edge::Left, true);
        window.set_anchor(gtk4_layer_shell::Edge::Right, true);
        window.set_anchor(gtk4_layer_shell::Edge::Top, true);
        window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        log::debug!("init");
        Ok(window)
    }
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let _ = Window::builder().application(app).build();

    // Present window
    //window.present();
}
