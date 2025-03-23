use crate::command;
mod wallpaper;
use gtk4::glib;
use std::sync::mpsc;
use std::time::Duration;
use std::{result, thread};
use wallpaper::Wallpaper;

/**
 * TODO rename
 */
pub(crate) struct Drawer<'a> {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<result::Result<(), command::ErrorReason>>,
}

impl<'a> Drawer<'a> {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<result::Result<(), command::ErrorReason>>,
    ) -> Self {
        Drawer {
            command_receiver,
            result_sender,
        }
    }

    /**
     *
     */
    pub(crate) fn run(self) -> result::Result<(), String> {
        let c = self.result_sender.clone();
        // Connect to "activate" signal of `app`
        let app = <gtk4::Application as wallpaper::Wallpaper>::new_application();
        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                loop {
                    log::debug!("running");
                    let temp = Temp {
                        command_receiver: &self.command_receiver,
                    }
                    .await;
                    log::debug!("temp: {:?}", temp);
                    c.send(Ok(()));
                    app.display();
                }
            }
        ));
        app.start();

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
        log::debug!("polling");
        match self.command_receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(command) => {
                log::debug!("received command: {:?}", command);
                std::task::Poll::Ready(command)
            }
            Err(_) => {
                log::debug!("not found");
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
