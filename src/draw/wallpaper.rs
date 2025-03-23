use gtk4::gio::ffi::G_RESOURCE_LOOKUP_FLAGS_NONE;
use gtk4::gio::ApplicationFlags;
use gtk4::glib::clone::Downgrade;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use std::sync::mpsc;
use std::time::Duration;
use std::{result, thread};

pub(crate) fn new_application() -> gtk4::Application {
    Application::builder()
        .application_id("dev.ryotaro.sheila")
        .build()
}

pub(crate) fn run_application(app: &gtk4::Application) -> glib::ExitCode {
    let args: &[String] = &[];
    // if run() is called, app interprets command line arguments
    app.run_with_args(args)
}

pub(crate) trait Wallpaper {
    fn new_application() -> impl Wallpaper;
    fn start(&self) -> glib::ExitCode;
    fn display(&self);
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> gtk4::Application {
        let app = Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build();
        app.connect_activate(build_ui);
        app
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
    }
    fn display(&self) {
        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(self)
            .title("My GTK App")
            .build();

        // Present window
        window.present();
    }
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    //window.present();
}
