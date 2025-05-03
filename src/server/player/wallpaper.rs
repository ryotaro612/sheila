use gstreamer;
use gtk4::{glib, Application, Window};
use gtk4::{prelude::*, Picture};
use gtk4_layer_shell::LayerShell;
use std::sync::{Arc, Mutex};

use crate::command::{self, make_server_error};

use super::monitor::detect_gdk_monitor;
use super::state;
use super::stream::Stream;
use super::window::{get_rectangle, init_window};

pub(crate) trait Wallpaper {
    ///
    fn new_application(state: &Arc<Mutex<state::State>>) -> Result<impl Wallpaper, String>;

    ///
    fn start(&self) -> glib::ExitCode;

    ///
    fn stop(&self);
    /// Terminate the application.
    fn shutdown(&self);
    ///
    fn play(
        &self,
        state: &Arc<Mutex<state::State>>,
        connector: &str,
        file: &str,
    ) -> Result<(), command::ErrorReason>;
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

    fn shutdown(&self) {
        self.windows().iter().for_each(|w| {
            w.close();
        });
        self.quit();
        unsafe {
            gstreamer::deinit();
        }
    }
    fn default_connector(&self) -> Result<String, String> {
        let monitor = detect_gdk_monitor(&None)?;
        monitor
            .connector()
            .map(|g| g.to_string())
            .ok_or("failed to resolve the connector of the default monitor".to_string())
    }
    ///
    fn new_application(state: &Arc<Mutex<state::State>>) -> Result<gtk4::Application, String> {
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();
        app.connect_activate(build_ui);

        gstreamer::init().map_err(|e| {
            app.quit();
            e.to_string()
        })?;

        state.lock().unwrap().set_app_running(true);
        let state1 = Arc::clone(&state);
        app.connect_shutdown(move |_| {
            let mut c = state1.lock().unwrap();
            c.set_app_running(true);
        });

        Ok(app)
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }

    fn stop(&self) {}

    fn play(
        &self,
        state: &Arc<Mutex<state::State>>,
        connector: &str,
        file: &str,
    ) -> Result<(), command::ErrorReason> {
        let gdk_monitor = detect_gdk_monitor(&Some(connector.to_string()))
            .map_err(|e| make_server_error(e.as_str()))?;

        let window: gtk4::Window =
            init_window(self, &gdk_monitor).map_err(|e| make_server_error(e.as_str()))?;

        let (width, height) = get_rectangle(&window).map_err(|e| {
            window.close();
            make_server_error(e.as_str())
        })?;

        let stream =
            Stream::new(file, width, height).map_err(|e| command::ErrorReason::ServerError {
                reason: e.to_string(),
            })?;

        state.lock().unwrap().add_stream(connector, &stream);
        let state1 = Arc::clone(&state);
        let connector1 = connector.to_string();
        window.connect_close_request(move |_| {
            let mut state = state1.lock().unwrap();
            state.stop_stream(&connector1);

            glib::Propagation::Proceed
        });

        let picture = Picture::for_paintable(&stream.paintable());
        window.set_child(Some(&picture));
        window.present();
        Ok(())
    }
}
/**
 *
 */
fn build_ui(app: &Application) {
    let _ = Window::builder().application(app).build();
}
