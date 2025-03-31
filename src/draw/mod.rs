use crate::command::{self, Command};
mod receiver;
mod wallpaper;
use crate::draw::receiver as dr;
mod monitor;

use gtk4::glib;
use std::result;
use std::sync::{self, mpsc};
use wallpaper::Wallpaper;

pub(crate) struct Drawer<'a> {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<Option<command::ErrorReason>>,
}

impl<'a> Drawer<'a> {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<Option<command::ErrorReason>>,
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
        let sender = self.result_sender.clone();
        // Connect to "activate" signal of `app`
        let app = <gtk4::Application as wallpaper::Wallpaper>::new_application();

        let arc_receiver = sync::Arc::new(sync::Mutex::new(self.command_receiver));

        //let arc_cmd_receiver = sync::Arc::new(sync::Mutex::new(self.command_receiver));
        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                let f = glib::clone!(
                    #[weak]
                    app,
                    move |c: Command| {
                        log::debug!("received command: {:?}", c);
                        let res = app.execute(c);
                        if let Err(e) = sender.send(res) {
                            log::error!("disconnected: {e}");
                            app.terminate();
                        }
                    }
                );
                loop {
                    let res = dr::ReceivedFuture::new(arc_receiver.clone()).await;
                    match res {
                        Ok(cmd) => f(cmd),
                        Err(r_err) => {
                            // https://doc.rust-lang.org/std/sync/mpsc/struct.RecvError.html
                            log::debug!("disconnected: {r_err}");
                            break;
                        }
                    }
                }
                app.terminate();
            }
        ));

        app.start();
        Ok(())
    }
}
