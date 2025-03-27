use crate::command;
mod receiver;
mod wallpaper;
use crate::draw::receiver as dr;
use gtk4::glib;
use std::future::{Future, IntoFuture};
use std::sync::mpsc;
use std::time::Duration;
use std::{result, thread};
use wallpaper::Wallpaper;

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

        let join_handle = glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                loop {
                    log::debug!("running");
                    receiver::ReceiveFuture {
                        receiver: &self.command_receiver,
                        interval: Duration::from_millis(300),
                    }
                    .await;

                    //log::debug!("temp: {:?}", temp);
                    c.send(Ok(()));
                    app.display();
                }
            }
        ));

        app.start();
        Ok(())
    }

    // /**
    //  *
    //  */
    // pub(crate) fn run(self) -> result::Result<(), String> {
    //     let c = self.result_sender.clone();
    //     // Connect to "activate" signal of `app`
    //     let app = <gtk4::Application as wallpaper::Wallpaper>::new_application();

    //     glib::spawn_future_local(glib::clone!(
    //         #[weak]
    //         app,
    //         async move {
    //             loop {
    //                 log::debug!("running");
    //                 let temp = Temp {
    //                     command_receiver: &self.command_receiver,
    //                     interval: Duration::from_millis(300),
    //                 }
    //                 .await;
    //                 log::debug!("temp: {:?}", temp);
    //                 c.send(Ok(()));
    //                 app.display();
    //             }
    //         }
    //     ));
    //     app.start();

    //     Ok(())
    // }
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

struct Temp<'a, T> {
    command_receiver: &'a mpsc::Receiver<T>,
    interval: Duration,
}

impl<'a, T> std::future::Future for Temp<'a, T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        log::debug!("polling");
        match self.command_receiver.recv_timeout(Duration::from_millis(0)) {
            Ok(command) => std::task::Poll::Ready(command),
            Err(_) => {
                log::debug!("not found");
                let waker = ctx.waker().clone();
                let interval = self.interval.clone();
                thread::spawn(move || {
                    thread::sleep(interval);
                    waker.wake_by_ref();
                });

                std::task::Poll::Pending
            }
        }
    }
}
