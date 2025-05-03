use gstreamer;
use gtk4::{glib, Application, Window};
use gtk4::{prelude::*, Picture};
use gtk4_layer_shell::LayerShell;

use crate::command::{self, make_server_error};

use super::monitor::detect_gdk_monitor;
use super::stream::Stream;
use super::window::{get_rectangle, init_window};

pub(crate) trait Wallpaper {
    ///
    fn new_application() -> Result<impl Wallpaper, String>;

    ///
    fn start(&self) -> glib::ExitCode;

    ///
    fn stop(&self);
    ///
    fn display(&self, connector: &str, file: &str) -> Result<Stream, command::ErrorReason>;
    ///
    fn default_connector(&self) -> Result<String, String>;
    ///
    fn close_window_by_connector(&self, connector: &str);
}

impl Wallpaper for gtk4::Application {
    fn close_window_by_connector(&self, connector: &str) {
        self.windows().iter().for_each(|w| {
            if let Some(m) = w.monitor() {
                if m.connector().unwrap_or_default() == connector {
                    w.close();
                }
            }
        });
    }
    fn default_connector(&self) -> Result<String, String> {
        let monitor = detect_gdk_monitor(&None)?;
        monitor
            .connector()
            .map(|g| g.to_string())
            .ok_or("failed to resolve the connector of the default monitor".to_string())
    }

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
    fn display(&self, connector: &str, file: &str) -> Result<Stream, command::ErrorReason> {
        let gdk_monitor = detect_gdk_monitor(&Some(connector.to_string()))
            .map_err(|e| make_server_error(e.as_str()))?;

        let window: gtk4::Window =
            init_window(self, &gdk_monitor).map_err(|e| make_server_error(e.as_str()))?;

        let (width, height) = get_rectangle(&window).map_err(|e| make_server_error(e.as_str()))?;

        let stream =
            Stream::new(file, width, height).map_err(|e| command::ErrorReason::ServerError {
                reason: e.to_string(),
            })?;

        let picture = Picture::for_paintable(&stream.paintable());

        //window.monitor().map(|m| m.geometry().width());
        window.set_child(Some(&picture));

        window.present();

        Ok(stream)
    }
}
/**
 *
 */
fn build_ui(app: &Application) {
    let _ = Window::builder().application(app).build();
}
