use gstreamer;
use gtk4::prelude::*;
use gtk4::{glib, Application, Window};
use gtk4_layer_shell::LayerShell;
use std::sync::{Arc, Mutex, Weak};

use crate::command::{make_invalid_params, make_server_error, ErrorReason};

use super::monitor::detect_gdk_monitor;
use super::state;
use super::window::{get_rectangle, init_window};

pub(crate) trait Wallpaper {
    ///
    fn new_application(state: &Arc<Mutex<state::State>>) -> Result<impl Wallpaper, String>;

    ///
    fn start(&self) -> glib::ExitCode;

    /// Terminate the application.
    fn shutdown(&self);
    ///
    fn play(
        &self,
        state: &Weak<Mutex<state::State>>,
        connector: &str,
        file: &str,
    ) -> Result<(), ErrorReason>;
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
        // An active gstreamer element can prevent the application from quitting.
        unsafe {
            gstreamer::deinit();
        }
        self.quit();
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

    fn play(
        &self,
        state: &Weak<Mutex<state::State>>,
        connector: &str,
        file: &str,
    ) -> Result<(), ErrorReason> {
        let gdk_monitor = detect_gdk_monitor(&Some(connector.to_string()))
            .map_err(|e| make_invalid_params(&e))?;

        let window: gtk4::Window =
            init_window(self, &gdk_monitor).map_err(|e| make_server_error(&e))?;

        let (width, height) = get_rectangle(&window).map_err(|e| {
            window.close();
            make_server_error(&e)
        })?;

        let on_error = glib::clone!(
            #[weak]
            window,
            move || {
                window.close();
            }
        );

        let arc_state = state
            .upgrade()
            .ok_or(make_server_error(&"The state was deleted."))?;
        let mut guard_state = arc_state.lock().unwrap();
        let picture = guard_state
            .add_stream(connector, file, width, height, on_error)
            .map_err(|e| make_server_error(&e))?;
        window.set_child(Some(&picture));

        let monitor_connector = connector.to_string();
        gdk_monitor.connect_invalidate(glib::clone!(
            #[weak]
            window,
            move |_: &gdk4::Monitor| {
                log::debug!("Monitor {monitor_connector} got invalidated");
                window.close();
            }
        ));
        let state1 = Weak::clone(&state);
        let connector1 = connector.to_string();
        window.connect_close_request(move |_| {
            if let Some(s) = state1.upgrade() {
                let mut state = s.lock().unwrap();
                state.stop_stream(&connector1);
            }
            glib::Propagation::Proceed
        });

        guard_state
            .play_stream(connector)
            .map_err(|e| {
                window.close();
                e
            })
            .map_err(|e| make_server_error(&e))?;

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
