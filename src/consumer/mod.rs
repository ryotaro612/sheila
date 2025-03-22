use crate::command;
use gtk4::gio::ffi::G_RESOURCE_LOOKUP_FLAGS_NONE;
use gtk4::glib::clone::Downgrade;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use std::result;
use std::sync::mpsc;

fn run_window() -> glib::ExitCode {
    const app_id: &str = "org.gtk_rs.HelloWorld";
    // Create a new application
    let app = Application::builder().application_id(app_id).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}

pub(crate) struct Consumer<'a> {
    command_receiver: &'a mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<result::Result<(), String>>,
}

async fn a() -> i32 {
    1
}

impl<'a> Consumer<'a> {
    pub(crate) fn new(
        command_receiver: &'a mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<result::Result<(), String>>,
    ) -> Self {
        Consumer {
            command_receiver,
            result_sender,
        }
    }

    /**
     *
     */
    pub(crate) fn run(self) -> result::Result<(), String> {
        const app_id: &str = "org.gtk_rs.HelloWorld";
        // Create a new application
        let app = Application::builder().application_id(app_id).build();

        // Connect to "activate" signal of `app`
        app.connect_activate(build_ui);

        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                app.windows();
                // while let Ok(enable_button) = receiver.recv().await {
                //     button.set_sensitive(enable_button);
                // }
            }
        ));

        // Run the application
        app.run();

        // loop {
        //     match self.command_receiver.recv() {
        //         Ok(command) => {
        //             thread::spawn(move || {

        //             });
        //         }
        //         Err(e) => {
        //             return Err(format!("error receiving a command: {e}"));
        //         }
        //     }
        //     break;
        // }
        Ok(())
    }
}

struct Temp {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: mpsc::Sender<result::Result<(), String>>,
}
impl Temp {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: mpsc::Sender<result::Result<(), String>>,
    ) -> Self {
        Temp {
            command_receiver,
            result_sender,
        }
    }

    /**
     *
     */
    pub(crate) fn run(self) -> result::Result<(), String> {
        const app_id: &str = "org.gtk_rs.HelloWorld";
        // Create a new application
        let app = Application::builder().application_id(app_id).build();

        // Connect to "activate" signal of `app`
        app.connect_activate(build_ui);
        glib::spawn_future_local(async move {
            self.command_receiver;
        });

        //   glib::spawn_future_local(glib::clone!(
        //         #[weak]
        //         app,
        //         async move {
        //             while let Ok(command) = self.command_receiver.recv() {
        //                 // Process the command or perform some action
        //                 //println!("Received command: {:?}", command);
        //             }
        //         }
        //     ));

        // Run the application
        app.run();

        // loop {
        //     match self.command_receiver.recv() {
        //         Ok(command) => {
        //             thread::spawn(move || {

        //             });
        //         }
        //         Err(e) => {
        //             return Err(format!("error receiving a command: {e}"));
        //         }
        //     }
        //     break;
        // }
        Ok(())
    }
}
