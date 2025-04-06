use gtk4::prelude::*;
use gtk4::{glib, Application, Window};
use gtk4_layer_shell::LayerShell;

use super::monitor::detect_gdk_monitor;

pub(crate) trait Wallpaper {
    fn new_application() -> impl Wallpaper;
    fn stop(&self);
    fn start(&self) -> glib::ExitCode;
    fn display(&self, monitor: &str, file: &str) -> Result<(), String>;
    fn find_window(&self, monitor: &str) -> Option<Window>;
    fn init_window(&self, monitor: &str) -> Result<Window, String>;
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> gtk4::Application {
        //app.connect_activate(build_ui);
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();

        //app.connect_activate(build_ui);
        app
    }

    fn stop(&self) {
        self.quit();
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }

    fn display(&self, connector: &str, file: &str) -> Result<(), String> {
        let window: gtk4::Window = match self.find_window(connector) {
            Some(window) => Ok::<Window, String>(window),
            None => match self.init_window(connector) {
                Ok(window) => Ok(window),
                Err(e) => {
                    return Err("failed to create a window".to_string());
                }
            },
        }?;
        Ok(())
    }

    fn find_window(&self, connector: &str) -> Option<Window> {
        let windows: Vec<Window> = self.windows();
        let found: Vec<Window> = windows
            .iter()
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
        window.set_layer(gtk4_layer_shell::Layer::Overlay);

        let monitor = detect_gdk_monitor(connector)?;
        window.fullscreen_on_monitor(&monitor);
        Ok(window)
    }
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    // let windows = app.windows();

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("My GTK App")
    //     .build();

    // Present window
    //window.present();
}
