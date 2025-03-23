use crate::command;
use gtk4::gio::ffi::G_RESOURCE_LOOKUP_FLAGS_NONE;
use gtk4::gio::ApplicationFlags;
use gtk4::glib::clone::Downgrade;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use std::sync::mpsc;
use std::time::Duration;
use std::{result, thread};

pub fn run_window() -> glib::ExitCode {
    const app_id: &str = "org.gtk_rs.HelloWorld";
    // Create a new application
    let app = Application::builder().application_id(app_id).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    let args: &[String] = &[];
    // if run() is called, app interprets command line arguments
    app.run_with_args(args)
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
/**
 * TODO rename
 */
pub(crate) struct Consumer<'a> {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<result::Result<(), command::ErrorReason>>,
}

impl<'a> Consumer<'a> {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<result::Result<(), command::ErrorReason>>,
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
        let app = Application::builder()
            .application_id(app_id)
            //.flags()
            .build();
        let c = self.result_sender.clone();
        // Connect to "activate" signal of `app`
        app.connect_activate(build_ui);

        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                //app.windows();
                loop {
                    //let command = self.command_receiver.recv();
                    let temp = Temp {
                        command_receiver: &self.command_receiver,
                    }
                    .await;

                    c.send(Ok(()));
                }
            }
        ));
        log::debug!("before application run");
        // Run the application
        let args: &[String] = &[String::from("")];
        // if run() is called, app interprets command line arguments
        app.run_with_args(args);
        log::debug!("after application run");
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

// struct Temp {
//     command_receiver: mpsc::Receiver<command::Command>,
//     result_sender: mpsc::Sender<result::Result<(), command::ErrorReason>>,
// }
// impl Temp {
//     pub(crate) fn new(
//         command_receiver: mpsc::Receiver<command::Command>,
//         result_sender: mpsc::Sender<result::Result<(), command::ErrorReason>>,
//     ) -> Self {
//         Temp {
//             command_receiver,
//             result_sender,
//         }
//     }

//     /**
//      *
//      */
//     pub(crate) fn run(self) -> result::Result<(), String> {
//         const app_id: &str = "org.gtk_rs.HelloWorld";
//         // Create a new application
//         let app = Application::builder().application_id(app_id).build();

//         // Connect to "activate" signal of `app`
//         app.connect_activate(build_ui);
//         glib::spawn_future_local(async move {
//             self.command_receiver;
//         });

//         //   glib::spawn_future_local(glib::clone!(
//         //         #[weak]
//         //         app,
//         //         async move {
//         //             while let Ok(command) = self.command_receiver.recv() {
//         //                 // Process the command or perform some action
//         //                 //println!("Received command: {:?}", command);
//         //             }
//         //         }
//         //     ));

//         // Run the application
//         app.run();

//         // loop {
//         //     match self.command_receiver.recv() {
//         //         Ok(command) => {
//         //             thread::spawn(move || {

//         //             });
//         //         }
//         //         Err(e) => {
//         //             return Err(format!("error receiving a command: {e}"));
//         //         }
//         //     }
//         //     break;
//         // }
//         Ok(())
//     }
// }

struct Temp<'a> {
    command_receiver: &'a mpsc::Receiver<command::Command>,
}

impl<'a> std::future::Future for Temp<'a> {
    type Output = command::Command;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        //log::debug!("polling");
        match self.command_receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(command) => std::task::Poll::Ready(command),
            Err(_) => {
                let waker = ctx.waker().clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(100));
                    waker.wake_by_ref();
                });

                std::task::Poll::Pending
            }
        }
    }
}
