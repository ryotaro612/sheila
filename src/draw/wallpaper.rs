use crate::draw::monitor;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};

pub(crate) trait Wallpaper {
    fn new_application() -> impl Wallpaper;
    fn stop(&self);
    fn start(&self) -> glib::ExitCode;
}

impl Wallpaper for gtk4::Application {
    fn new_application() -> gtk4::Application {
        //app.connect_activate(build_ui);
        Application::builder()
            .application_id("dev.ryotaro.sheila")
            .build()
    }

    fn stop(&self) {
        self.quit();
    }

    fn start(&self) -> glib::ExitCode {
        let args: &[String] = &[];
        // if run() is called, app interprets command line arguments
        self.run_with_args(args)
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
