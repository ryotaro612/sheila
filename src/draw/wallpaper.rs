use crate::draw::monitor;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};

pub(crate) trait Wallpaper {
    fn new_application() -> impl Wallpaper;
    fn application(&self) -> &gtk4::Application;
    fn start(&self) -> glib::ExitCode;
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> gtk4::Application {
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();
        app.connect_activate(build_ui);
        app
    }

    fn application(&self) -> &gtk4::Application {
        self
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }
}

fn build_ui(app: &Application) {
    // Create a window and set the title

    let monitors = monitor::detect_monitors();
    let windows = app.windows();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    //window.present();
}
